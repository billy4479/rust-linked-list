mod just_a_module {
    use crate::linked_list;

    pub fn just_a_function() {
        let mut l = linked_list::List::<i32>::new();
        l.push(1);

        let mut i = l.iter();
        let x = i.next();

        x.map(|y| {
            println!("{}", y);
        });
    }
}
