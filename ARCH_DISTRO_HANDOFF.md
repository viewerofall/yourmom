# Yo Mama Linux - Complete Arch Distro Implementation

**"btw I use yo mama"**

This document outlines how to build a complete, production-ready Arch Linux distribution where the init system, core utilities, and system tools are written in .yourmom, while maintaining 100% compatibility with standard Arch packages and workflows.

---

## Architecture Overview

```
Yo Mama Linux = Arch Base + daddy_init + Compatibility Layer + Theming
```

### What We Keep (Stock Arch):
- ✅ Linux kernel (standard)
- ✅ pacman package manager
- ✅ All official repos (core, extra, multilib)
- ✅ GNU coreutils (as fallback)
- ✅ GCC, binutils, build tools
- ✅ All standard Arch packages

### What We Add:
- ✅ `yourmom` compiler (pre-installed)
- ✅ `daddy_init` (init system, PID 1)
- ✅ `mamactl` (service control)
- ✅ Compatibility layer (`systemctl` → `mamactl`)
- ✅ Optional .yourmom utilities
- ✅ Desktop theming (Niri + purple TWM)

### What We Build:
- ✅ Init system in .yourmom
- ✅ Service manager in .yourmom
- ✅ System utilities (optional)
- ✅ Desktop environment configs

---

## Phase 1: daddy_init - Init System

### **Goal:**
Replace systemd with a .yourmom init that's:
- Fast as runit (no D-Bus overhead)
- Easy as systemd (`mamactl start nginx`)
- 100% compatible with existing packages

### **File: daddy_init.yourmom**

```yourmom
// daddy_init.yourmom - PID 1 init system
// "Yo mama so fundamental, she's process 1"

yo mama_main() {
    yo_mama_so_loud("👨 DADDY'S HOME (PID 1)")
    
    // Mount essential filesystems
    dym mount_proc()
    dym mount_sys()
    dym mount_dev()
    dym mount_run()
    
    // Load services from /etc/mama/services/
    yo services = dym load_all_services()
    
    // Start enabled services with dependencies
    dym start_boot_target(services)
    
    // Main loop - reap zombies and monitor services
    mama_keeps_saying 1 == 1 {
        dym reap_zombies()
        dym check_crashed_services()
        dym handle_ipc_messages()
        dym sleep_ms(100)
    }
}

yo load_all_services() {
    yo service_dir = "/etc/mama/services/"
    yo files = dym list_directory(service_dir)
    yo services = dym array_new()
    
    mama_told_everyone file in files {
        mama_said dym string_ends_with(file, ".service") {
            yo service = dym parse_service_file(file)
            dym array_push(services, service)
        }
    }
    
    did_your_mom services
}

yo spawn_service(service) {
    yo pid = dym fork_process()
    
    mama_said pid == 0 {
        // Child process
        dym setuid(service.user)
        dym chdir(service.working_dir)
        dym exec_command(service.command)
    } mama_lied {
        // Parent - track PID
        dym register_service_pid(service.name, pid)
        yo_mama_so_loud("👶 Birth:")
        yo_mama_so_loud(service.name)
    }
}

yo check_crashed_services() {
    yo crashed = dym get_dead_services()
    
    mama_told_everyone service in crashed {
        mama_said service.restart == "always" {
            yo_mama_so_loud("💀 Service died, respawning:")
            yo_mama_so_loud(service.name)
            dym sleep_seconds(service.restart_delay)
            dym spawn_service(service)
        }
    }
}

yo reap_zombies() {
    // waitpid(-1, NULL, WNOHANG)
    dym c_waitpid_nonblock()
}

yo handle_ipc_messages() {
    yo sock = dym get_ipc_socket()
    yo msg = dym socket_recv_nonblock(sock)
    
    mama_said msg != daddy_left {
        yo parts = dym string_split(msg, ":")
        yo command = parts[0]
        yo service_name = parts[1]
        
        mama_said command == "START" {
            dym spawn_service_by_name(service_name)
        }
        
        mama_said command == "STOP" {
            dym kill_service_by_name(service_name)
        }
        
        mama_said command == "RESTART" {
            dym kill_service_by_name(service_name)
            dym sleep_ms(100)
            dym spawn_service_by_name(service_name)
        }
    }
}
```

---

## Phase 2: mamactl - Service Control

### **File: mamactl.yourmom**

```yourmom
// mamactl.yourmom - Service control CLI
// "Yo mama controls everything"

yo mama_main() {
    yo args = dym get_all_args()
    
    mama_said dym array_len(args) < 2 {
        dym print_usage()
        did_your_mom 1
    }
    
    yo command = args[1]
    
    mama_said command == "start" {
        dym cmd_start(args[2])
    }
    
    mama_said command == "stop" {
        dym cmd_stop(args[2])
    }
    
    mama_said command == "restart" {
        dym cmd_restart(args[2])
    }
    
    mama_said command == "status" {
        dym cmd_status(args[2])
    }
    
    mama_said command == "enable" {
        dym cmd_enable(args[2])
    }
    
    mama_said command == "disable" {
        dym cmd_disable(args[2])
    }
    
    mama_said command == "list" {
        dym cmd_list()
    }
    
    mama_said command == "reload" {
        dym cmd_reload()
    }
}

yo cmd_start(service_name) {
    yo_mama_so_loud("👶 Starting:")
    yo_mama_so_loud(service_name)
    dym send_to_init("START", service_name)
}

yo cmd_stop(service_name) {
    yo_mama_so_loud("💀 Stopping:")
    yo_mama_so_loud(service_name)
    dym send_to_init("STOP", service_name)
}

yo cmd_status(service_name) {
    yo pid = dym read_pid_file(service_name)
    
    mama_said pid > 0 {
        mama_said dym process_exists(pid) {
            ymf(service_name)
            ymf(" is alive (PID: ")
            ymf(pid)
            ymf(")")
        } mama_lied {
            ymf(service_name)
            ymf(" is dead (daddy left)")
        }
    } mama_lied {
        ymf(service_name)
        ymf(" is not running")
    }
}

yo cmd_enable(service_name) {
    yo source = "/etc/mama/services/"
    yo target = "/etc/mama/enabled/"
    dym string_append(source, service_name)
    dym string_append(source, ".service")
    dym string_append(target, service_name)
    dym string_append(target, ".service")
    dym symlink(source, target)
    ymf("Enabled ")
    ymf(service_name)
}

yo cmd_disable(service_name) {
    yo target = "/etc/mama/enabled/"
    dym string_append(target, service_name)
    dym string_append(target, ".service")
    dym unlink(target)
    ymf("Disabled ")
    ymf(service_name)
}

yo send_to_init(command, service) {
    yo sock = dym connect_socket("/run/daddy_init.sock")
    yo msg = command
    dym string_append(msg, ":")
    dym string_append(msg, service)
    dym socket_send(sock, msg)
    dym close_socket(sock)
}
```

---

## Phase 3: systemd Compatibility Layer

### **Goal:**
Make ALL systemd commands work transparently.

### **File: /usr/bin/systemctl (wrapper script)**

```bash
#!/bin/bash
# systemctl compatibility wrapper
# Translates systemd commands to mamactl

MAMACTL="/usr/bin/mamactl-real"

case "$1" in
    start|stop|restart|status|enable|disable)
        exec $MAMACTL "$@"
        ;;
    
    is-active)
        $MAMACTL status "$2" | grep -q "alive"
        exit $?
        ;;
    
    is-enabled)
        test -L "/etc/mama/enabled/$2.service"
        exit $?
        ;;
    
    list-units)
        exec $MAMACTL list
        ;;
    
    daemon-reload)
        exec $MAMACTL reload
        ;;
    
    --version)
        echo "systemd compatibility layer (backed by daddy_init)"
        echo "Yo mama so compatible, she pretends to be systemd"
        exit 0
        ;;
    
    *)
        echo "Unknown systemd command: $1"
        echo "Supported: start stop restart status enable disable is-active is-enabled list-units daemon-reload"
        exit 1
        ;;
esac
```

### **Symlinks:**

```bash
ln -s /usr/bin/systemctl-compat /usr/bin/systemctl
ln -s /usr/bin/systemctl-compat /usr/bin/service
ln -s /usr/bin/mamactl-compat /usr/bin/journalctl
```

---

## Phase 4: Service File Auto-Conversion

### **Pacman Hook: /usr/share/libalpm/hooks/mama-convert.hook**

```ini
[Trigger]
Operation = Install
Operation = Upgrade
Type = File
Target = usr/lib/systemd/system/*.service

[Action]
Description = Converting systemd services to mama format
When = PostTransaction
Exec = /usr/bin/mama-auto-convert
```

### **File: /usr/bin/mama-auto-convert**

```bash
#!/bin/bash
# Auto-convert systemd services to mama format

for svc in /usr/lib/systemd/system/*.service; do
    name=$(basename "$svc")
    mama_svc="/etc/mama/services/$name"
    
    # Skip if already converted
    [ -f "$mama_svc" ] && continue
    
    # Parse systemd file
    desc=$(grep "^Description=" "$svc" | cut -d= -f2-)
    exec_start=$(grep "^ExecStart=" "$svc" | cut -d= -f2-)
    user=$(grep "^User=" "$svc" | cut -d= -f2-)
    restart=$(grep "^Restart=" "$svc" | cut -d= -f2-)
    after=$(grep "^After=" "$svc" | cut -d= -f2- | sed 's/.target/.service/g')
    
    # Generate mama service
    cat > "$mama_svc" <<EOF
[Family]
Name=${name%.service}
Description=${desc:-No description}

[Birth]
Command=${exec_start}
${user:+User=$user}

[Death]
${restart:+Restart=$restart}

[Relationships]
${after:+After=$after}
EOF
    
    echo "Converted: $name"
done

# Reload init
mamactl reload 2>/dev/null || true
```

---

## Phase 5: C Interop Library

### **File: libmama.c - System call wrappers**

```c
// libmama.c - C wrappers for system calls needed by .yourmom

#include <unistd.h>
#include <sys/wait.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <fcntl.h>
#include <dirent.h>

// Process management
int c_fork_process(void) {
    return fork();
}

int c_exec_command(const char *cmd) {
    return execl("/bin/sh", "sh", "-c", cmd, NULL);
}

int c_waitpid_nonblock(void) {
    return waitpid(-1, NULL, WNOHANG);
}

int c_process_exists(int pid) {
    return kill(pid, 0) == 0;
}

// Socket IPC
int c_create_socket(const char *path) {
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    struct sockaddr_un addr = {0};
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, path, sizeof(addr.sun_path) - 1);
    unlink(path);
    bind(fd, (struct sockaddr*)&addr, sizeof(addr));
    listen(fd, 5);
    return fd;
}

int c_connect_socket(const char *path) {
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    struct sockaddr_un addr = {0};
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, path, sizeof(addr.sun_path) - 1);
    connect(fd, (struct sockaddr*)&addr, sizeof(addr));
    return fd;
}

// File operations
char** c_list_directory(const char *path) {
    // TODO: Return array of filenames
    return NULL;
}

int c_symlink(const char *target, const char *linkpath) {
    return symlink(target, linkpath);
}

// Mount operations
int c_mount_proc(void) {
    return mount("proc", "/proc", "proc", 0, NULL);
}

int c_mount_sys(void) {
    return mount("sysfs", "/sys", "sysfs", 0, NULL);
}

// etc...
```

---

## Phase 6: archiso Configuration

### **Directory Structure:**

```
archiso-yomama/
├── airootfs/           # Root filesystem overlay
│   ├── etc/
│   │   ├── mama/
│   │   │   ├── services/
│   │   │   │   ├── network.service
│   │   │   │   ├── sshd.service
│   │   │   │   └── niri.service
│   │   │   └── enabled/
│   │   ├── pacman.conf
│   │   └── skel/
│   │       └── .config/
│   │           └── niri/
│   │               └── config.kdl
│   ├── usr/
│   │   ├── bin/
│   │   │   ├── yourmom
│   │   │   ├── mamactl-real
│   │   │   ├── systemctl-compat
│   │   │   └── mama-auto-convert
│   │   └── share/
│   │       ├── yourmom/
│   │       │   ├── examples/
│   │       │   └── stdlib/
│   │       └── wallpapers/
│   └── sbin/
│       └── daddy_init
│
├── packages.x86_64     # Packages to include
├── pacman.conf
└── profiledef.sh       # ISO metadata
```

### **packages.x86_64:**

```
# Base system
base
linux
linux-firmware
grub
efibootmgr

# Build tools
gcc
make
binutils

# Yourmom compiler (from AUR)
yourmom

# Desktop
niri
kitty
waybar
wofi

# Fonts & theme
noto-fonts
noto-fonts-emoji
ttf-fira-code
papirus-icon-theme

# Utilities
neofetch
htop
vim
git

# Network
networkmanager
openssh
```

### **profiledef.sh:**

```bash
#!/usr/bin/env bash

iso_name="yomama"
iso_label="YOMAMA_$(date +%Y%m)"
iso_publisher="Yo Mama Linux <https://github.com/viewerofall/yomama>"
iso_application="Yo Mama Linux Live"
iso_version="$(date +%Y.%m.%d)"
install_dir="arch"
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito' 'uefi-x64.systemd-boot.esp' 'uefi-x64.systemd-boot.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/sbin/daddy_init"]="0:0:755"
  ["/usr/bin/mamactl-real"]="0:0:755"
)
```

---

## Phase 7: Build & Distribution

### **Build ISO:**

```bash
# Clone archiso
git clone https://gitlab.archlinux.org/archlinux/archiso.git
cd archiso

# Copy our profile
cp -r /path/to/archiso-yomama configs/yomama

# Build ISO
sudo mkarchiso -v -w /tmp/archiso-tmp configs/yomama
```

### **GitHub Repo Structure:**

```
yomama-linux/
├── compiler/           # yourmom compiler source
├── init/               # daddy_init + mamactl
├── iso/                # archiso config
├── packages/           # PKGBUILD files
│   ├── yourmom/
│   ├── family-init/
│   └── yomama-theme/
└── docs/               # Installation guide
```

---

## Phase 8: Installation Experience

### **First Boot Menu:**

```
Yo Mama Linux 2025.03
btw I use yo mama

1. Boot Yo Mama Linux (x86_64, UEFI)
2. Boot Yo Mama Linux (x86_64, BIOS)
3. Boot existing OS
```

### **Installer (archinstall profile):**

```python
# /usr/lib/python3.11/site-packages/archinstall/profiles/yomama.py

class YoMamaProfile(Profile):
    def install(self):
        # Install base
        install_packages(['base', 'linux', 'linux-firmware'])
        
        # Install yourmom compiler
        install_aur_package('yourmom')
        
        # Install init system
        install_packages(['family-init'])
        
        # Install desktop
        install_packages(['niri', 'kitty', 'waybar'])
        
        # Configure theme
        copy_config('/etc/skel/.config/niri', '/home/user/.config/niri')
        
        # Enable services
        run_command('mamactl enable network')
        run_command('mamactl enable sshd')
        run_command('mamactl enable niri')
```

---

## Implementation Timeline

### **Week 1: Core Init System**
- ✅ Write daddy_init.yourmom
- ✅ Write mamactl.yourmom
- ✅ Write libmama.c (C syscall wrappers)
- ✅ Compile and test in VM

### **Week 2: Compatibility Layer**
- ✅ systemctl wrapper script
- ✅ Service file auto-converter
- ✅ Pacman hook
- ✅ Test with real packages (nginx, postgresql, docker)

### **Week 3: ISO Build**
- ✅ archiso configuration
- ✅ Theme and wallpapers
- ✅ Default configs (Niri, Kitty)
- ✅ Build and test ISO

### **Week 4: Documentation & Polish**
- ✅ Installation guide
- ✅ User documentation
- ✅ GitHub repo setup
- ✅ Release ISO

---

## Features Checklist

### **Init System:**
- ✅ PID 1 written in .yourmom
- ✅ Service management
- ✅ Dependency resolution
- ✅ Crash recovery
- ✅ IPC socket
- ✅ Zombie reaping

### **Compatibility:**
- ✅ `systemctl` works
- ✅ `service` works
- ✅ AUR packages install
- ✅ Docker works
- ✅ Auto-convert .service files

### **Desktop:**
- ✅ Niri compositor
- ✅ Purple TWM theme
- ✅ Custom wallpapers
- ✅ Kitty terminal
- ✅ Waybar status bar

### **Distribution:**
- ✅ Custom ISO
- ✅ archinstall profile
- ✅ GitHub repo
- ✅ Documentation

---

## Success Criteria

The distro is complete when:
1. ✅ ISO boots in VM
2. ✅ archinstall works
3. ✅ `systemctl start nginx` works
4. ✅ `pacman -S docker && systemctl enable docker` works
5. ✅ AUR packages install normally
6. ✅ Desktop environment loads
7. ✅ .yourmom compiler works out of the box

---

## Next Steps for Claude Code

1. **Implement daddy_init.yourmom** (300-400 lines)
2. **Implement mamactl.yourmom** (200-300 lines)
3. **Write libmama.c** (syscall wrappers)
4. **Test in VM** (VirtualBox/QEMU)
5. **Build archiso** once init works
6. **Package for GitHub**

---

## Notes

- Keep GNU coreutils as fallback
- Don't reinvent pacman
- Focus on init + compatibility
- Test with real packages early
- Document everything

**"Yo mama so compatible, she runs Arch packages without breaking a sweat."**
