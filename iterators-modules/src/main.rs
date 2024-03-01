struct FilterCondition<T> {
    condition: T
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

fn custom_filter<T>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T> 
where 
    T: PartialEq
{
    collection.into_iter().filter(|item| condition.is_match(item)).collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let condition = FilterCondition { condition: 3 };
    let filtered_numbers = custom_filter(numbers, &condition);
    
    println!("Filtered numbers: {:?}", filtered_numbers);
}