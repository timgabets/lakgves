%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: lakgves
Summary: Framework for testing Bank credit card processing systems (ISO8583)
Version: @@VERSION@@
Release: @@RELEASE@@
License: MIT
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%config(noreplace) %attr(640, root, wheel) %{_sysconfdir}/lakgves/config.toml
%defattr(-,root,root,-)
%{_bindir}/*