// data structure
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String))
    
}



// ================================================================================================
// function as paramater
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}
fn square(value: i32) -> i32 {
    value * value
}
fn cube(value: i32) -> i32 {
    value * value * value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        assert_eq!(4, apply(2, square));
        assert_eq!(8, apply(2, cube));
    }
}