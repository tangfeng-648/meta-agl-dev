# Remove selinux-python dependency from policycoreutils to fix force install python runtime issue.

RDEPENDS:${BPN}:remove = "selinux-python"
