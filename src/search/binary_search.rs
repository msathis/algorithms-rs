fn find(vec: Vec<i32>, n: i32) -> i32 {
    let len = vec.len();
    return bsearch(vec, n, 0, (len - 1) as i32);
}

fn bsearch(vec: Vec<i32>, n: i32, l: i32, h: i32) -> i32 {
    let mid = (l + h) / 2;

    if l > h {
        return -1;
    }

    if n == *vec.get(mid as usize).unwrap() {
        return mid;
    } else if n < *vec.get(mid as usize).unwrap() {
        return bsearch(vec, n, l, mid - 1);
    }
    return bsearch(vec, n, mid + 1, h);
}

mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find(vec![0, 1, 3, 6, 8, 9, 16], 8), 4);
        assert_eq!(find(vec![0, 1, 3, 6, 8, 9, 16], 4), -1);
    }
}