Name: ohc
Version: 0
Release: 1
Summary: One Human Corp application bundle
License: Apache-2.0

%description
One Human Corp application bundle

%prep

%build

%install
mkdir -p "%{buildroot}/usr/local/bin"
cp "../ohc-builtin-agent" "%{buildroot}/usr/local/bin/ohc-builtin-agent"
cp "../server" "%{buildroot}/usr/local/bin/ohc-server"
chmod 0755 "%{buildroot}/usr/local/bin/ohc-builtin-agent" "%{buildroot}/usr/local/bin/ohc-server"

%files
/usr/local/bin/ohc-builtin-agent
/usr/local/bin/ohc-server
