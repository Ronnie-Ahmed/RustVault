use std::{collections::HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration,Instant};

#[derive(Clone)]
pub struct ValueWithExpiry{
     value:String,
     expires_at:Option<Instant>
}

impl ValueWithExpiry {
    fn is_expired(&self) -> bool {
        self.expires_at.is_some_and(|t| Instant::now() >= t)
    }
}


#[derive(Clone)]
pub struct Db{
    shared : Arc<RwLock<HashMap<String,ValueWithExpiry>>>,
}

impl Db{
   pub fn new() -> Self {
        Db {
            shared: Arc::new(RwLock::new(HashMap::new())),
        }

        
    }

    pub fn start_background_cleaner(&self, interval: Duration) {
        let db_clone = self.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;
                db_clone.purge_expired_keys().await;
            }
        });
    }
    pub async fn set(&self,key:String,value:String,ttl:Option<Duration>){
        let mut map=self.shared.write().await;
        let expires_at=ttl.map(|d|Instant::now() + d);
        map.insert(key, ValueWithExpiry { value, expires_at });
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        {
            let map = self.shared.read().await;
            match map.get(key) {
                Some(entry) if !entry.is_expired() => return Some(entry.value.clone()),
                Some(_) => {} 
                None => return None,
            }
        }

        let mut map = self.shared.write().await;
        if let Some(entry) = map.get(key) {
            if entry.is_expired() {
                map.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        } else {
            None
        }
    }

    pub async fn delete(&self, key: &str) -> bool {
        let mut map = self.shared.write().await;
        map.remove(key).is_some()
    }
    async fn purge_expired_keys(&self) {
        let mut map = self.shared.write().await;
        map.retain(|_, entry| !entry.is_expired());
    }

    pub async fn len(&self) -> usize {
        self.shared.read().await.len()
    }

}