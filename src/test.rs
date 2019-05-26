#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    #[should_panic]
    fn test_show_field() {
        let mut test_field = [[0; SIZE_X];SIZE_Y];
        test_field[1][5] = 5;
        show_field(&test_field);
    }

    #[test]
    fn test_winner_2() {
        let mut test_field = [[0; SIZE_X];SIZE_Y];
        test_field = [[1,1,1,1,1,1,1,1],
                      [1,1,1,1,1,1,1,1],
                      [2,2,2,2,2,2,2,2],
                      [2,2,2,2,2,2,2,2],
                      [1,2,1,2,1,2,1,2],
                      [0,0,2,2,0,0,1,2],
                      [3,2,2,2,2,1,1,0],
                      [0,3,1,1,1,2,2,2]];
        assert_eq!(winner(&test_field), 2);
    }

    #[test]
    fn test_winner_1() {
        let mut test_field = [[0; SIZE_X];SIZE_Y];
        test_field = [[1,2,1,0,1,1,1,1],
                      [1,2,1,0,1,1,1,1],
                      [2,1,2,1,2,6,2,0],
                      [2,1,2,2,1,2,1,2],
                      [1,2,1,1,1,2,1,2],
                      [0,0,0,2,0,0,1,0],
                      [3,0,2,2,0,1,1,0],
                      [0,0,1,1,1,2,1,2]];
        assert_eq!(winner(&test_field), 1);
    }

    #[test]
    fn test_winner_tie() {
        let mut test_field = [[0; SIZE_X];SIZE_Y];
        test_field = [[1,1,1,1,1,1,1,1],
                      [1,1,1,1,1,1,1,1],
                      [2,2,2,2,2,2,2,2],
                      [2,2,2,2,2,2,2,2],
                      [1,2,1,2,1,2,1,2],
                      [1,0,2,2,1,0,1,2],
                      [3,2,2,2,2,1,1,0],
                      [1,1,1,1,1,2,2,2]];
        assert_eq!(winner(&test_field), 0);
    }
}
