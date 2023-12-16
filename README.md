<p align="center">
  <img src="https://user-images.githubusercontent.com/67526259/236649035-8215ae72-2b3d-4fa3-ada5-674390a46db2.png" alt="hax logo">
  <br><br>
  A Remake of <a href="https://github.com/skyline69/hax">Hax</a> in Rust™
  <br><br>
  <img src="https://github.com/skyline69/HaxRS/assets/67526259/260b37b2-640a-414d-9da4-1dd90d6593d3" draggable="false" width="550px" style="user-select:none;" alt="demo picture">
<br><br>
</p>

<br>

![9aad62f6a86fa29a5bde2ab9f647d22c](https://user-images.githubusercontent.com/67526259/236649115-7c9dc679-53dc-416c-935c-07961eac59d2.png)
### - Port-scanner(Powered by nmap) ✅
### - URL-Masker ✅
### - Phisher(Powered by ZPhisher) ☑️(WIP 🚧)
> Note: HaxRS now works on **macOS** too! 🥳
> 
<br><br>

<img src="https://user-images.githubusercontent.com/67526259/236820904-73b10112-6094-4938-9d89-85d9cf7811d5.png" alt="For Linux">
<br>

> For Linux Enthusiasts: You **need** to install **`nmap`**.


## How to install and build HaxRS on Linux & macOS
## Quick install (Linux & macOS)
Copy and paste the following command into your terminal to install HaxRS:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/skyline69/HaxRS/main/install.sh | bash
```
## Manual install (Only for Linux)
### Step 1: Install Rust (If you haven't already)
1. Open the terminal by pressing `Ctrl + Alt + T` or searching for "Terminal" in the application menu.
2. Install Rust by running the following command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Verify the installation by running the following command:
```bash
rustc --version
```
### Step 2: Install nmap (If you haven't already)
> Look at the <a href="#how-to-install-nmap-on-linux">here</a> for instructions on how to install nmap on your Linux distribution.
### Step 3: Clone the repository
Clone the repository by running the following command:
```bash
git clone https://github.com/skyline69/HaxRS.git
```
### Step 4: Build the project
Navigate to the project directory by running the following command:
```bash
cd HaxRS
```
Build the project by running the following command:
```bash
cargo build --release
```
### Step 5: Run the project
Navigate to the project directory by running the following command:
```bash
cd HaxRS
```
Run the project by executing the following command:
```bash
./target/release/haxrs
```

---

### Examples of `nmap` installation
- <a href="#ubuntu">Ubuntu</a>
- <a href="#debian">Debian</a>
- <a href="#arch-linux">Arch Linux</a>
- <a href="#fedora">Fedora</a>
- <a href="#centos">CentOS</a>
- <a href="#opensuse">OpenSUSE</a>
- <a href="#gentoo">Gentoo</a>

## How to install `nmap` on macOS
> You need to have Homebrew installed. If you don't have it installed, you can install it by running the following command:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

1. Open the terminal by pressing `Cmd + Space` and searching for "Terminal".
2. Install `nmap` by running the following command:
```bash
brew install nmap
```
3. Verify the installation by running the following command:
```bash
nmap --version
```



## How to install `nmap` on Linux
### Ubuntu
1. Open the terminal by pressing `Ctrl + Alt + T` or searching for "Terminal" in the application menu.
2. Update the package list by running the following command:
```bash
sudo apt update
```
3. Install nmap by running the following command:
```bash
sudo apt install nmap
```
4. Verify the installation by running the following command:
```bash
nmap --version
```
### Debian
1. Open the terminal by pressing `Ctrl + Alt + T` or searching for "Terminal" in the application menu.
2. Update the package list by running the following command:
```bash
sudo apt update
```
3. Install nmap by running the following command:
```bash
sudo apt install nmap
```
4. Verify the installation by running the following command:
```bash
nmap --version
```
### Arch Linux
1. Open the terminal by pressing `Ctrl + Alt + T` or searching for "Terminal" in the application menu.
2. Update the package list by running the following command:
```bash
sudo pacman -Syu
```
3. Install nmap by running the following command:
```bash
sudo pacman -S nmap
```
4. Verify the installation by running the following command:
```bash
nmap --version
```
### Fedora
1. Open the terminal by pressing `Ctrl + Alt + T` or searching for "Terminal" in the application menu.
2. Update the package list by running the following command:
```bash
sudo dnf update
```
3. Install nmap by running the following command:
```bash
sudo dnf install nmap
```
4. Verify the installation by running the following command:
```bash
nmap --version
```
### CentOS
1. Open the terminal by pressing `Ctrl + Alt + T` or searching for "Terminal" in the application menu.
2. Update the package list by running the following command:
```bash
sudo yum update
```
3. Install nmap by running the following command:
```bash
sudo yum install nmap
```
4. Verify the installation by running the following command:
```bash
nmap --version
```
### OpenSUSE
1. Open the terminal by pressing `Ctrl + Alt + T` or searching for "Terminal" in the application menu.
2. Update the package list by running the following command:
```bash
sudo zypper update
```
3. Install nmap by running the following command:
```bash
sudo zypper install nmap
```
4. Verify the installation by running the following command:
```bash
nmap --version
```
### Gentoo
1. Open the terminal by pressing `Ctrl + Alt + T` or searching for "Terminal" in the application menu.
2. Update the package list by running the following command:
```bash
sudo emerge --sync
```
3. Install nmap by running the following command:
```bash
sudo emerge net-analyzer/nmap
```
4. Verify the installation by running the following command:
```bash
nmap --version
```
## Disclaimer
This software is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the author or copyright holder be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

Please note that this tool is meant for educational purposes only. The author or any contributors are not responsible for any actions, illegal or otherwise, performed by anyone who uses this tool. Any actions and or activities related to the material contained within this software is solely your responsibility.

It is the end user's responsibility to obey all applicable local, state and federal laws. Developers assume no liability and are not responsible for any misuse or damage caused by this program.
