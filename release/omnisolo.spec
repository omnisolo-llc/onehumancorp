Name: ohc
Version: 0
Release: 1
Summary: OmniSolo application bundle
License: Apache-2.0

%description
OmniSolo application bundle

%prep

%build

%install
mkdir -p "%{buildroot}/usr/local/bin"
cp "../{omnisolo-builtin-agent}" "%{buildroot}/usr/local/bin/omnisolo-builtin-agent"
cp "../{server}" "%{buildroot}/usr/local/bin/omnisolo-server"
chmod 0755 "%{buildroot}/usr/local/bin/omnisolo-builtin-agent" "%{buildroot}/usr/local/bin/omnisolo-server"

%files
/usr/local/bin/omnisolo-builtin-agent
/usr/local/bin/omnisolo-server
