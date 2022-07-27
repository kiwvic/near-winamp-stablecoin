use crate::*;

pub fn vector_to_vec<T> (vector: &Vector<T>) -> Vec<T> 
where T: BorshDeserialize 
{
    let mut result = vec![];

    for val in vector.iter() {
        result.push(val);
    }

    return result;
}
