# ipfs_contract

# prerequisites

1. Install curl : 
sudo apt install curl

2. Install Rust : 
Type the following command and follow the on screen instructions:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3. Install IPFS  :
wget https://dist.ipfs.io/go-ipfs/v0.9.1/go-ipfs_v0.9.1_linux-amd64.tar.gz
tar -xvzf go-ipfs_v0.9.1_linux-amd64.tar.gz
cd go-ipfs
sudo bash install.sh

4. Start IPFS server : 
ipfs daemon
