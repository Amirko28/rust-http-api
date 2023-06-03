use crate::model::user::User;

pub fn get_data() -> Vec<User> {
    return vec![
        User::new("Paul".into(), 1685210064397),
        User::new("Ringo".into(), 1285210064397),
        User::new("George".into(), 1185210064397),
        User::new("John".into(), 1625010014397)
    ];
}