#!/usr/bin/env python3
"""Render deployment files and verify control/storage isolation across containers."""
import pathlib, subprocess, tempfile, unittest, yaml
ROOT=pathlib.Path(__file__).resolve().parents[2]
class LocalServices(unittest.TestCase):
    def test_compose_each_worker_has_private_daemon(self):
        result=subprocess.run(['docker','compose','-f',str(ROOT/'deploy/docker-compose.yml'),'-f',str(ROOT/'deploy/docker-compose.local-services.yml'),'config','--no-interpolate','--no-consistency'],capture_output=True,text=True,check=True)
        services=yaml.safe_load(result.stdout)['services']
        workers=[name for name in services if name.startswith('harness-') and not name.endswith('-local-services')]
        self.assertEqual(len(workers),12)
        for name in workers:
            worker=services[name]; daemon=services[name+'-local-services'] if name+'-local-services' in services else None
            if name.endswith('-local-services'):continue
            self.assertIsNotNone(daemon)
            for container in (worker,daemon):
                if isinstance(container['environment'],list): container['environment']=dict(item.split('=',1) for item in container['environment'])
            self.assertEqual(daemon['network_mode'],'service:'+name)
            self.assertNotIn('pid',daemon)
            self.assertEqual(worker['environment']['OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN'],daemon['environment']['OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN'])
            self.assertEqual(worker['environment']['OMNISOLO_LOCAL_SERVICE_ENDPOINT'],'http://127.0.0.1:8095')
            self.assertFalse(any(v['target'].startswith('/services') for v in worker.get('volumes',[])))
            self.assertTrue(any(v['target']=='/services' for v in daemon['volumes']))
            self.assertFalse(any(v['target']=='/workspace' for v in daemon['volumes']))
    def test_helm_renders_daemon_only_storage_and_secret_control(self):
        worker={'id':'test','harnessId':'codex','poolId':'test','autoscaling':{'enabled':False},'localServices':{'image':'daemon:test','configMap':'admitted-config','dataClaim':'service-data','controlSecretName':'service-control','controlSecretKey':'token'}}
        with tempfile.NamedTemporaryFile(mode='w',suffix='.yaml') as values:
            yaml.safe_dump({'backend':{'grpcTls':{'existingSecret':'tls'},'auth':{'existingSecret':'auth'}},'powersync':{'enabled':False},'harnessWorkers':{'enabled':True,'workers':[worker]}},values);values.flush()
            output=subprocess.check_output(['helm','template','test',str(ROOT/'deploy/helm/omnisolo'),'-f',values.name,'--show-only','templates/harness-workers.yaml'],text=True)
        deployment=next(d for d in yaml.safe_load_all(output) if d['kind']=='Deployment')
        pod=deployment['spec']['template']['spec']; containers={c['name']:c for c in pod['containers']}
        self.assertEqual(set(containers),{'harness-worker','local-services'})
        self.assertFalse(pod.get('shareProcessNamespace',False))
        for name in containers:
            token=next(e for e in containers[name]['env'] if e['name']=='OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN')
            self.assertEqual(token['valueFrom']['secretKeyRef'],{'name':'service-control','key':'token'})
        mounts={m['name'] for m in containers['harness-worker']['volumeMounts']}
        self.assertNotIn('local-service-data',mounts);self.assertNotIn('local-service-config',mounts)
        mounts={m['name'] for m in containers['local-services']['volumeMounts']}
        self.assertIn('local-service-data',mounts);self.assertIn('local-service-config',mounts)
if __name__=='__main__': unittest.main()
