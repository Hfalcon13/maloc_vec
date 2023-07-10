pub mod m_vec;

use m_vec::MVec;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add() {
        let mut mvec: MVec<i32> = MVec::new();

        mvec.add(10);
        mvec.add(20);
        mvec.add(30);

        assert_eq!(mvec.count, 3);
        assert_eq!(mvec.capacity, 16); // The initial capacity is 0, so it should have expanded to 16 after adding elements.

        let values = unsafe { std::slice::from_raw_parts(mvec.values, mvec.count) };

        assert_eq!(values, &[10, 20, 30]);
    }

    #[test]
    fn test_add_expand_capacity() {
        let mut mvec: MVec<i32> = MVec::new();

        for i in 0..20 {
            mvec.add(i);
        }

        assert_eq!(mvec.count, 20);
        assert_eq!(mvec.capacity, 32);

        let values = unsafe { std::slice::from_raw_parts(mvec.values, mvec.count) };

        assert_eq!(values, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
    }

    #[test]
    fn test_add_expand_capacity_multiple_types() {
        let mut mvec: MVec<String> = MVec::new();

        mvec.add("Hello".to_string());
        mvec.add("World".to_string());
        mvec.add("Rust".to_string());

        assert_eq!(mvec.count, 3);
        assert_eq!(mvec.capacity, 16);

        let values = unsafe { std::slice::from_raw_parts(mvec.values, mvec.count) };

        assert_eq!(values, &[
            "Hello".to_string(),
            "World".to_string(),
            "Rust".to_string(),
        ]);
    }

    use rand::Rng;
    #[test]
    fn test_abunch_of_alocations() {
        let mut v: MVec<i32> = MVec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(200..=10000) {
            v.add(rng.gen());
        }

    }


   

}
