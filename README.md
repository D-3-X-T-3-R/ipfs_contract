# ipfs_contract

### prerequisites

#### 1. Install curl :
sudo apt install curl

#### 2. Install Rust :
Type the following command and follow the on screen instructions:<br />
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#### 3. Install IPFS  :
wget https://dist.ipfs.io/go-ipfs/v0.9.1/go-ipfs_v0.9.1_linux-amd64.tar.gz <br />
tar -xvzf go-ipfs_v0.9.1_linux-amd64.tar.gz <br />
cd go-ipfs <br />
sudo bash install.sh <br />

#### 4. Start IPFS server :
run ipfs daemon

#### 5. Install ink smart contract: <br />
run cargo install cargo-contract --force <br />

#### 6. Change the rust version to nightly by typing 'rutup default nightly' <br />

### Running the program
1. Edit the ipfs_contract/run.sh to pass the desired file as argument.
2. Execute the run script by typing ./run.sh

### Testing the smart contract
1. cd ipfs_contract/src/storage/
2. Run cargo +nightly test

