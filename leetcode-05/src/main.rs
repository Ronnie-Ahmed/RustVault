
pub fn longest_palindrome(s:String)->String{
    let b= s.as_bytes();
    let n=b.len() as isize;
    let mut start:isize=0;
    let mut max_len:isize=0;

    let expand=|mut left:isize,mut right:isize|->isize{
        while left>=0 && right<n && b[left as usize]==b[right as usize]{
            left-=1;
            right+=1;
        }
        right-left-1
    };

    for i in 0..n{
        let odd=expand(i,i);
        let even=expand(i,i+1);
        let len=odd.max(even);

        if len>max_len{
            max_len=len;
            start=i-(len-1)/2;
        }
    }

    s[start as usize..(start+max_len) as usize].to_string()



    
   
}

fn main() {
    println!("{}",longest_palindrome("babad".to_string()));
}
