// Глобальная статическая переменная
static MAX_USERS: u32 = 100_000 ;

use std::thread ;

// Тут 'static не значит «замыкание живёт вечно». 
// Значит «замыкание не захватывает короткоживущих ссылок». 
// String подходит под T: 'static потому что владеет своими данными.
fn spawn<F: FnOnce() + 'static + Send>(f: F) {
    thread::spawn(move || {
        f() ;
    })
    .join()
    .unwrap() ;
}

fn main() {
    let s: &'static str = "abc" ;

    let leaked: &'static mut String = 
        // "Протекает" Box — превращает его в статическую 
        // ссылку &'static mut String
        // Функция потребляет и удаляет объект Box, возвращая изменяемую 
        // ссылку &'a mut T.
        // Обратите внимание, что тип T должен существовать дольше выбранного
        // времени жизни 'a'. Если тип имеет только статические ссылки 
        // или не имеет их вовсе, то можно выбрать значение 'static'.    
        Box::leak(
            Box::new(   // Помещает эту строку в умный указатель Box
                // Создает владеющую строку "efj" в куче
                String::from("efj")
            )
        ) ;

    println!("{}, {}, {}", MAX_USERS, s, leaked) ;  // 100000, abc, efj

    let data = String::from("klm") ;

    spawn(
        move || {
            println!("Get data: {data}") ;
        }
    );  // Out: Get data: klm

}
