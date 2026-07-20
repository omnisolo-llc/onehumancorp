/self\.memory\.store\(/ {
  print "            /* self.memory.store("
  print "                &event.tenant_id,"
  print "                \"Operations\","
  print "                &format!(\"Parsed offline voice intent: {}\", transcription)"
  print "            ).await?; */"
  in_skip = 1
  next
}
in_skip == 1 && /^\s*\)\.await\?;/ {
  in_skip = 0
  next
}
in_skip == 1 { next }

/if let Some\(pool\) = crate::db::get_pool_opt\(\)/ {
  print "            let pool = crate::db::get_pool();"
  print "            {"
  next
}

{ print }
