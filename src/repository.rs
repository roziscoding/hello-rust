use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use uuid::Uuid;

use crate::models::{Friend, NewFriend};

pub type Db = Arc<RwLock<HashMap<Uuid, Friend>>>;

#[derive(Clone)]
pub struct FriendRepository {
    db: Db,
}

impl FriendRepository {
    pub fn new() -> Self {
        return FriendRepository {
            db: Arc::new(RwLock::new(HashMap::new())),
        };
    }

    #[cfg(test)]
    pub fn seed(friends: HashMap<Uuid, Friend>) -> Self {
        return FriendRepository {
            db: Arc::new(RwLock::new(friends)),
        };
    }

    fn read(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Friend>> {
        return self.db.read().unwrap();
    }

    fn write(&self) -> RwLockWriteGuard<'_, HashMap<Uuid, Friend>> {
        return self.db.write().unwrap();
    }

    pub fn list(&self) -> Vec<Friend> {
        return self.read().values().cloned().collect();
    }

    pub fn get(&self, id: &Uuid) -> Option<Friend> {
        return self.read().get(id).cloned();
    }

    pub fn delete(&self, id: &Uuid) {
        self.write().remove(id);
    }

    pub fn add(&self, friend_params: NewFriend) -> Friend {
        let id = Uuid::new_v4();
        let new_friend = Friend {
            id,
            name: friend_params.name,
            pronouns: friend_params.pronouns,
            notes: friend_params.notes,
        };

        self.write().insert(id, new_friend.clone());

        return new_friend;
    }

    pub fn update(&self, id: &Uuid, params: NewFriend) -> Option<Friend> {
        let mut friends = self.write();
        let friend = friends.get_mut(id)?;

        friend.name = params.name;
        friend.pronouns = params.pronouns;
        friend.notes = params.notes;

        return Some(friend.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_friend() -> (Uuid, Friend) {
        let id = Uuid::new_v4();
        let friend = Friend {
            id,
            name: "Friend".into(),
            pronouns: "Pronouns".into(),
            notes: None,
        };

        return (id, friend);
    }

    #[test]
    fn it_lists_friends() {
        let mut friends: HashMap<Uuid, Friend> = HashMap::new();
        let (uuid, friend) = make_friend();
        friends.insert(uuid, friend);

        let repo = FriendRepository::seed(friends);
        let friends = repo.list();
        assert_eq!(friends.len(), 1);
    }

    #[test]
    fn it_adds_a_friend() {
        let repo = FriendRepository::new();

        let new_friend = repo.add(NewFriend {
            name: "Test friend".into(),
            pronouns: "Test Pronouns".into(),
            notes: None,
        });

        assert_eq!(new_friend.name, "Test friend");

        let friend = repo.get(&new_friend.id);

        assert!(friend.is_some());
        assert_eq!(friend.unwrap().name, "Test friend");
    }

    #[test]
    fn update_returns_none_on_wrong_id() {
        let repo = FriendRepository::new();

        let result = repo.update(
            &Uuid::new_v4(),
            NewFriend {
                name: String::new(),
                pronouns: String::new(),
                notes: None,
            },
        );

        assert!(result.is_none());
    }

    #[test]
    fn it_updates_when_called_correctly() {
        let (uuid, friend) = make_friend();
        let mut friends: HashMap<Uuid, Friend> = HashMap::new();
        friends.insert(uuid, friend);
        let repo = FriendRepository::seed(friends);

        repo.update(
            &uuid,
            NewFriend {
                name: "Updated".into(),
                pronouns: "Updated".into(),
                notes: Some("Present".into()),
            },
        );

        let updated_friend = repo.get(&uuid).unwrap();

        assert!(updated_friend.notes.is_some());
        assert_eq!(updated_friend.notes.unwrap(), "Present");
        assert_eq!(updated_friend.name, "Updated");
        assert_eq!(updated_friend.pronouns, "Updated");
    }
}
