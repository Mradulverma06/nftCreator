#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, String, Symbol, symbol_short};

const COUNT : Symbol = symbol_short!("COUNTER");

#[contracttype]
pub enum Mapnft {DigiNft(u64)}

#[contracttype]
#[derive(Clone)]
pub struct DigiNft {id: u64,owner: String,ipfs: String,}

#[contract]
pub struct Nftcreator;
#[contractimpl]
impl Nftcreator {
    pub fn createnft(env: Env, owr: String, ipfs: String) -> u64 {
        let mut nft_count: u64 = env.storage().instance().get(&COUNT).unwrap_or(0);
            nft_count += 1;

        let mut nftoverview = Self::getnft(env.clone(), nft_count.clone());
        
        nftoverview.id = nft_count;
        nftoverview.owner = owr;
        nftoverview.ipfs = ipfs;

        env.storage().instance().set(&Mapnft:: DigiNft(nftoverview.id.clone()), &nftoverview);
        env.storage().instance().set(&COUNT, &nftoverview.id.clone());
        env.storage().instance().extend_ttl(5000, 5000);

        return nftoverview.id ;        
    }

    
    pub fn getnft(env: Env, id: u64) -> DigiNft {
        let key = Mapnft::DigiNft(id.clone());

        env.storage().instance().get(&key).unwrap_or(DigiNft {
            id: 0,
            owner: String::from_str(&env, "Invalid"),
            ipfs: String::from_str(&env, "Invalid"),
        })
    }
}