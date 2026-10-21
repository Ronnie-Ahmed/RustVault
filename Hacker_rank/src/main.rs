//----------------------------------------------------
//--------Time Conversion-----------------------------

fn timeConversion(s: &str)->String {
    let  temp: Vec<char> = s.chars().collect();
    let mut extension_part = String::new();
    let mut hours = String::new();
    let mut result=String::new();
    let mut count = 0;
    for i in &temp {
        if *i == 'P' || *i == 'A' || *i == 'M' {
            extension_part.push(*i);
        }

        if count < 2 {
            hours.push(*i);
            count += 1;
        }
    }

    let mut _n_hours=hours.parse::<i32>().unwrap();
    if extension_part=="PM"{
        if _n_hours >=1 && _n_hours<=11{
            _n_hours+=12;
            result.push_str(&_n_hours.to_string());
        }else{
            result.push_str(&_n_hours.to_string());
        }
    }else{
        if _n_hours >=1 && _n_hours<=11{
            result.push_str(&_n_hours.to_string());
        }else{
            result.push_str(&"00".to_string());
        }
    }

     for i in 0..temp.len() {
        if i>1 && i<8{
            result.push(temp[i]);
        }
    }
    if result.len() !=8{
        result.insert(0, '0');
    }
    result

}

fn timeConversion2(s:&str)->String{
    let hour:u32=s[0..2].parse().unwrap();
    let offset= if s.ends_with("PM"){12}else {
        0
    };
    format!("{:02}{}",hour%12+offset,&s[2..8])
}

//----------------------------------------------------
//----------------------------------------------------


fn modifySequence(arr: &[i32]) -> i32 {
     let mut tails: Vec<i32> = Vec::new(); // LNDS via binary search

    for (i, &x) in arr.iter().enumerate() {
        let b = x - (i as i32 + 1);
        if b < 0 {
            continue;
        }
        // first index where tails[idx] > b (non-decreasing allows equal)
        let idx = tails.partition_point(|&t| t <= b);
        if idx == tails.len() {
            tails.push(b);
        } else {
            tails[idx] = b;
        }
    }

    (arr.len() - tails.len()) as i32
}

fn gradingStudents(grades: &[i32])  {
    let mut temp=Vec::new();
    for n in grades{
        if *n>37{
            let modu=n%5;
            if modu >= 3 && modu!=0{

                temp.push(n+(5-modu));
            }else if modu ==0 {
                temp.push(*n);
            }else {
                temp.push(*n);
            }

        }else {
            temp.push(*n);
        }
    }
    println!("{:?}",temp);
}


pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        
        
        let mut n_target=target as u8;

        if n_target >=letters[letters.len()-1] as u8{
            return letters[0]
        }
        for c in letters{
            let value=c as u8;
            if value > n_target{
               n_target=value;
               break;
            }
        }
        n_target as char
                
}


 pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
       let n = cost.len();

        // prev2 = dp[i-2], prev1 = dp[i-1]
        let mut prev2 = 0;
        let mut prev1 = 0;

        for i in 2..=n {
            let current = std::cmp::min(
                prev1 + cost[i - 1],
                prev2 + cost[i - 2],
            );
            prev2 = prev1;
            prev1 = current;
        }

        prev1 // dp[n]
}


fn main() {
//    println!("{}",timeConversion("07:05:45PM"));
//    println!("{}",timeConversion2("07:05:45PM"));
//-------------------------------------------------

// println!("{:?}",modifySequence(&[1,2,8,4]));
//-------------------------------------------------
// gradingStudents(&[73,67,38,33]);
// println!("{:?}",next_greatest_letter(vec!['c','f','j'],'a'));
println!("{:?}",min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]));
// let a=vec![1,2,3];
// println!("{}",a.len());



}
