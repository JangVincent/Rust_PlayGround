# Rust PlayGround

---

### This README.md contained Korean translated content of [official site](https://doc.rust-lang.org/book/)

---

# Variables in Rust

`mut` 키워드는 변경 가능한 값을 의미한다.

```rust
let mut x = 5;
x = 10 //ok

let y = 5
y = 10 // unavailable

```

`const` 는 변경불가능한 상수를 뜻한다.

```rust
const x : u32 = 60 * 60 * 3;
x = 2345 //unavailable
```

## Shadowing

위에서 선언했던 변수와 같은 이름으로 아래에서 선언되어 사용되어지는 것을
러스트 이용자들(Rustaceans)은 섀도잉 이라고 부른다.

```rust
let x = 5;

let x = x + 1; // 6

{
    let x = x * 2; //12
    println!("The value of x in the inner scope is: {x}");
}

println!("The value of x is: {x}"); //6

```

새로운 블럭 내에서 위의 x를 이용해서 let키워드로 x를 재정의 해 사용하는 것을 볼 수 있다.  
그렇게 선언된 x는 종속된 코드블럭 내에서 유효하며, 범위를 벗어나면 다시 원래의 x값을 가지게 된다.  
사실상 컨텍스트 스위칭에 의한 새로운 변수로 볼 수 있겠다.

새도잉은 변수를 mut 키워드를 사용해 정의 하는것과는 다르다.  
우리는 `let` 키워드를 사용하지 않고 갑자기 다른 값을 할당하려고 한다면 컴파잍타임 에러를 불러 일으킨다.

let을 이용하면, 우리는 몇몇 가자의 값의 변환을 할 수 있지만, 섀도잉이 끝난 이후의 변수는 불변상태(immutable) 상태가 된다.

# Data Type in Rust

러스트의 모든 값은 자료형을 가지게 된다.  
이것은 러스트에게 어떤 종류의 데이터가 확정되어지는 지 알려주며, 이것은 데이터와 어떻게 작업 될 지 알 수 있다.

우리는 여기서 크게 두가지 자료형 부분집합 `Scalar`와 `Compound`를 알아볼 것이다.

일단 러스트는 정적 타입언어임을 항상 머릿속에 기억하는 것이 좋다.  
이것은 컴파일 타임에 모든 자료형을 알아야만 한다는 것을 뜻한다.  
컴파일러는 실제 값과, 우리가 어떻게 사용하는지를 바탕으로 우리가 어떻게 사용하고싶은가를 대개 유추할 수 있다.

여러 자료형이 가능할 경우, (예를 들어 우리가 문자열 자료형을 parse 를 통해 숫자자료형으로 바꾸는 때와 같이), 우리는 다음과 같이 타입 설정을 반드시 해주어야 한다.

```rust

let guess: u32 = "42".parse().expect("Not a number!");

```

## Scalar Types

스칼라 자료형은 기본적으로 단일 값을 표현한다. 러스트는 4개의 원시 스칼라 자료형을 지원한다.

-   integer
-   float-point number
-   boolean
-   characters

만약 이것에 대해서 다른 프로그래밍 언어에서 본 적이 있어 익숙하다면 이것들이 어떻게 러스트에서 쓰이는지 알아보도록 하자.

### 정수 자료형

![정수 자료형](./readme/integer-types.png)

정수 자료형은 명시적인 크기를 가지며, 부호가 있을 수도 없을 수도 있습니다.

이 때, `isize`, `usize` 타입들은 은 당신의 프로그램이 돌아가는 컴퓨터의 구조에 따라 달라질 수 있다.  
당신이 64비트 컴퓨터를 사용 할 경우 i/u64, 32비트 컴퓨터의 경우 i/u32 의 메모리를 할당받개 된다. ㄴ

정수 데이터를 명시, 표기 할 때 여러 방식으로 표기할 수 있다.  
이것을 정수 리터럴(Integer literal) 이라고 하며 다음과 같이 표기할 수 있다.  
(이 때, 여러 숫자 유형이 될 수 있다면, `58u8` 과 같이 유형 접미사를 허용함.)  
뿐만아니라 숫자를 읽기 쉽도록 `_` (언더바) 로 구분할 수 있다.

![정수 리터럴](./readme/integer-literal.png)

정수 유형의 기본 값은 i32 이며,`isize`, `usize` 등을 사용하는 주요 상황은 어떤 컬렉션을 인덱싱 할 때 이다.

### 부동 소수점 자료형

러스트는 물론 두개의 부동 소수점 자료형을 지원합니다. 십진수 소수점을 가짐.  
`f32`, `f64` 두가지 자료형을 지원하며, 기본은 `f64` 를 사용.  
최근 CPU 는 처리속도는 비슷하지만 정확도가 더 높기 때문.

모든 숫자 자료형의 사칙연산은 C의 사칙연산과 같음

-   / : 몫을 반환
-   % : 나머지 반환

### 불리언 자료형

다른 프로그래밍 랭귀지 처럼 러스트도 불리언 타입을 지원.  
1바이트의 사이즈를 가지며, 타입을 명시할 시 `bool` 키워드를 사용.

```rust

let mut t : bool = true;
```

### 캐릭터 자료형(문자)

작은 따옴표로 리터럴 지정합니다. 크기가 4바이트. (기존 C와 같은 언어는 1바이트)  
유니코드 스칼라 값을 가지기 때문에 이모티콘 같은 것들, 한국어, 일본어 등을 지원.

## Compound Types

-   Tuple
-   Array

### Tuple

튜플은 다영한 여러 유형의 값을 하나로 그롭화 하는 일반적 방법

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

```

튜플은 단일 복합요소로 간주되어, tup 라는 변수에 바인딩 됨.  
따라서 별개의 밧을 얻고 싶은 경우 다음과 같은 매칭 패턴을 사용할 수 있음.

```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {y}");

```

자바스크립트에서의 일종의 구조분해할당(비구조화 문법) 을 생각할 수 있겠다.  
또는 각 값을 점`.`을 사용해 인덱스 처럼 접근 할 수 있는데 다음과 같다

```rust

let x :(i32, f64, u8) = (500, 6.7, 1);

println!("1st value of tuple : {x.0}")

```

### Arary

튜플과 달리 모든 원소가 같은 데이터 타입이어야 하는 배열도 있다.
일반적으로는 C언어의 그것과 같으나 다른점을 찾아보자.

러스트는 타입을 변수에 써 줄 수 있는데, 배열은 다음처럼 사용한다.

```rust

let a: [i32; 5] = [1, 2, 3, 4, 5];
let three : [3; 5]; //[3,3,3,3,3]

```

## Function in Rust

```rust

// 선언
fn function_name () {

}

// 매개변수
fn function(x : isize) {
    println!("Parameter x is {x}")
}

```

러스트의 함수 본문은 선택적으로 표현식으로 끝낼 수 있다.  
러스트는 표현식 기반 언어이다.  
다른언어에는 명령문과 표현식에 대한 구분이 없으나, 러스트에는 있으며, 어떤 영항을 가지는지 알아보자.

statement 는 어떤 액션을 취하고 어떠한 것도 반환하지 않는 명령.  
expression 은 결과값으로 평가(인식)됨.

예시는 다음과 같다.

```rust
let a = 1 //statement

let b = (let a = 1) // 이 동작은 새로운 변수에 명령문을 대입한 것과 같으므로 에러 발생

let c = {
    let b = 10;
    a + b  // a = 1, b = 10 -> 11
}; // 이것은 표현식이고, 반환값으로 (결과값으로 ) 인식되므로 11을 c에 대입한 것과 같음
```

이제 함수를 리턴값과 같이 사용해보자.

```rust

fn returnFive() -> i8 {
    5 // 세미콜로늘 붙이지 않으면 함수의 리턴값으로 간주됨
}

fn main() {
    let x = returnFive();

    println!("x is : {x}")
}

```

## Flow Control in Rust

### if / else expression

```rust
    let a = 10;


    if a {

    } // error occurred
```

러스트는 다른언어와 달리 불리언이 아닌 유형을 자동으로 변환하지 않음.  
명시적어야하며 `if` 문과 같은 조건문들은 항상 `bool` 값을 condition으로 제공해야 함.

제목에서와 같이 if문은 표현식이므로 변수에 할당 할 수 있음

```rust

let mut condition : bool : true
let a = if condition {5} else {10} ;

```

### loop expression

다른 언어와 같이 반복문을 지원하는데 조금 특이하다.  
loop 역시 표현식이므로 올바르게 값을 리턴한다면 변수에 값을 지정할 수 있다.

```rust
let mut counter = 0
let a = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
}; //a = 20
```

중첩된 루프와 같이 여러 루프를 레이블(label) 을 지원한다.

```rust
'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

```

이때 다른 언어와 같이 `break`, `continue` 와 같은 문법을 지원하는데,  
이 때, 레이블이 지정되지 않았다면, `break`, `continue` 가 속한 가장 안쪽 루프에 대하여 작동한다.

### conditional loop : while

다른 언어에서의 그것과 같음

```rust

let mut a = 1;

while a < 10 {
    println!("{a}");
    a+=1;
}

```

### for with collection types

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}


for i in (1..10).rev()  { //rev() is reverse()
    println!("{i}");
}
```
