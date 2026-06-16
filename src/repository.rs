use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set};
use crate::entities::{friend, Friend};

use uuid::Uuid;

use crate::models::{NewFriend};

#[derive(Clone)]
pub struct FriendRepository {
    db: DatabaseConnection,
}

impl FriendRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        return FriendRepository {
            db
        };
    }

    pub async fn list(&self) -> Result<Vec<Friend>, DbErr> {
        return friend::Entity::find().all(&self.db).await;
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Friend>, DbErr> {
        return friend::Entity::find_by_id(id).one(&self.db).await;
    }

    pub async fn delete(&self, id: Uuid) -> Result<sea_orm::DeleteResult, DbErr> {
        return friend::Entity::delete_by_id(id).exec(&self.db).await;
    }

    pub async fn add(&self, friend_params: NewFriend) -> Result<Friend, DbErr> {
        let friend = Friend::new(
            friend_params.name,
            friend_params.pronouns,
            friend_params.notes,
        );

        return friend.insert(&self.db).await;
    }

    pub async fn update(&self, id: Uuid, params: NewFriend) -> Result<Option<Friend>, DbErr> {
        let friend = friend::Entity::find_by_id(id).one(&self.db).await?;

        if let Some(friend) = friend {
            let mut friend = friend.into_active_model();

            friend.name = Set(params.name);
            friend.pronouns = Set(params.pronouns);
            friend.notes = Set(params.notes);

            return Ok(Some(friend.update(&self.db).await?));
        };

        return Ok(None);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn make_friend() -> (Uuid, Friend) {
//         let id = Uuid::new_v4();
//         let friend = Friend {
//             id,
//             name: "Friend".into(),
//             pronouns: "Pronouns".into(),
//             notes: None,
//         };

//         return (id, friend);
//     }

//     #[test]
//     fn it_lists_friends() {
//         let mut friends: HashMap<Uuid, Friend> = HashMap::new();
//         let (uuid, friend) = make_friend();
//         friends.insert(uuid, friend);

//         let repo = FriendRepository::seed(friends);
//         let friends = repo.list();
//         assert_eq!(friends.len(), 1);
//     }

//     #[test]
//     fn it_adds_a_friend() {
//         let repo = FriendRepository::new();

//         let new_friend = repo.add(NewFriend {
//             name: "Test friend".into(),
//             pronouns: "Test Pronouns".into(),
//             notes: None,
//         });

//         assert_eq!(new_friend.name, "Test friend");

//         let friend = repo.get(&new_friend.id);

//         assert!(friend.is_some());
//         assert_eq!(friend.unwrap().name, "Test friend");
//     }

//     #[test]
//     fn update_returns_none_on_wrong_id() {
//         let repo = FriendRepository::new();

//         let result = repo.update(
//             &Uuid::new_v4(),
//             NewFriend {
//                 name: String::new(),
//                 pronouns: String::new(),
//                 notes: None,
//             },
//         );

//         assert!(result.is_none());
//     }

//     #[test]
//     fn it_updates_when_called_correctly() {
//         let (uuid, friend) = make_friend();
//         let mut friends: HashMap<Uuid, Friend> = HashMap::new();
//         friends.insert(uuid, friend);
//         let repo = FriendRepository::seed(friends);

//         repo.update(
//             &uuid,
//             NewFriend {
//                 name: "Updated".into(),
//                 pronouns: "Updated".into(),
//                 notes: Some("Present".into()),
//             },
//         );

//         let updated_friend = repo.get(&uuid).unwrap();

//         assert!(updated_friend.notes.is_some());
//         assert_eq!(updated_friend.notes.unwrap(), "Present");
//         assert_eq!(updated_friend.name, "Updated");
//         assert_eq!(updated_friend.pronouns, "Updated");
//     }
// }
