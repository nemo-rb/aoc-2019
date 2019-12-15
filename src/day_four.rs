#[cfg(test)]
mod test_four {
    use super::{check_password, check_password_2, run_1, run_2};

    #[test]
    fn test_good_password() {
        assert!(
            check_password(111111)
        );
    }


    #[test]
    fn test_bad_password() {
        assert!(
            !check_password(223450)
        );
    }


    #[test]
    fn test_bad_password_1() {
        assert!(
            !check_password(123789)
        );
    }


    #[test]
    fn test_good_password_1() {
        assert!(
            check_password_2(112233)
        );
    }


    #[test]
    fn test_bad_password_2() {
        assert!(
            !check_password_2(123444)
        );
    }


    #[test]
    fn test_good_password_2() {
        assert!(
            check_password_2(111122)
        );
    }


    #[test]
    fn test_run_1() {
        assert_eq!(
            run_1(), 1660
        );
    }


    #[test]
    fn test_run_2() {
        assert_eq!(
            run_2(), 1135
        );
    }
}


pub fn run_1() -> i64 {
    let input = 172_851..675_869;

    input.filter(|&i| check_password(i)).count() as i64
}


pub fn run_2() -> i64 {
    let input = 172_851..675_869;

    input.filter(|&i| check_password_2(i)).count() as i64
}


fn check_password(password: i64) -> bool {
    check_length(password) && check_ascending(password) && check_duplicates(password)
}


fn check_password_2(password: i64) -> bool {
    check_length(password) && check_ascending(password) && check_duplicates_count(password)
}


fn check_length(password: i64) -> bool {
    password >= 100_000 && password <= 999_999
}


fn check_ascending(password: i64) -> bool {
    let mut pass = password;
    let mut previous;

    previous = pass % 10;
    pass /= 10;

    while pass != 0 {
        let lsd = pass % 10;

        if lsd > previous {
            return false;
        }

        previous = lsd;
        pass /= 10;
    }

    true
}


fn check_duplicates(password: i64) -> bool {
    let mut pass = password;
    let mut duplicate = false;
    let mut previous;

    previous = pass % 10;
    pass /= 10;

    while pass != 0 {
        let lsd = pass % 10;

        if lsd == previous {
            duplicate = true;
        }

        previous = lsd;
        pass /= 10;
    }

    duplicate
}


fn check_duplicates_count(password: i64) -> bool {
    let mut pass = password;
    let mut count = 1;
    let mut previous;

    previous = pass % 10;
    pass /= 10;

    while pass != 0 {
        let lsd = pass % 10;

        if lsd == previous {
            count += 1;
        } else {
            if count == 2 {
                return true
            }

            count = 1;
        }

        previous = lsd;
        pass /= 10;
    }

    count == 2
}
