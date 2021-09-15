%define debug_package %{nil}

Name: moonrabbit
Summary: Implementation of a https://moonrabbit.com node
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

Requires: systemd, shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}


%prep
%setup -q


%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%post
config_file="/etc/default/moonrabbit"
getent group moonrabbit >/dev/null || groupadd -r moonrabbit
getent passwd moonrabbit >/dev/null || \
    useradd -r -g moonrabbit -d /home/moonrabbit -m -s /sbin/nologin \
    -c "User account for running moonrabbit as a service" moonrabbit
if [ ! -e "$config_file" ]; then
    echo 'MOONRABBIT_CLI_ARGS=""' > /etc/default/moonrabbit
fi
exit 0

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/moonrabbit.service
