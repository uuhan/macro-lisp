#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
//#![feature(trace_macros)]

#[macro_use]
extern crate macro_lisp;

#[cfg(test)]
mod tests {
    use macro_lisp::*;

    #[test]
    fn test_progn(){
        lisp!(progn
            (defconstant x 3)
            (defconstant y 4)
            (defvar z (* x y))
            (assert_eq 12 z)
        );
    }

    #[test]
    fn test_if(){
        lisp!(if (eq 1 1) (println "equal"));
        lisp!(if (eq 2 2) (println "equal") (println "not equal"));
        let x = lisp!(if true (+ 1 1) (+ 2 2));
        assert_eq!(2, x);

        lisp!(if (<= 1 2) (println "True") (println "False"));
    }

    lisp!(defun hello () ()
        (println "Hello")
    );

    lisp!(defun add ((x i32) (y i32)) i32
        (+ x y)
    );

    lisp!(defun do_nothing());

    //trace_macros!(true);

    #[test]
    fn test_defun(){
        let x = lisp!(add 3 4);
        assert_eq!(7, x);

        do_nothing();
    }

    //trace_macros!(false);

    #[test]
    fn test_macro_utils(){
        lisp!(print "hello, {}" "world");
    }

    #[test]
    fn test_defconstant(){
        lisp!(defconstant x 3);
        assert_eq!(3, x);

        lisp!(defconstant cell (cons 3 4));
        assert_eq!(3, lisp!(car cell));
        assert_eq!(4, lisp!(cdr cell));

        lisp!(defconstant (y &str) "hello");
    }

    #[test]
    fn test_defvar(){
        lisp!(defvar x 0);
        assert_eq!(0, x);
        lisp!(setf x 1);
        assert_eq!(1, x);

        lisp!(defvar cell (cons 5 "test"));
        assert_eq!(5, lisp!(car cell));
        assert_eq!("test", lisp!(cdr cell));

        lisp!(setf cell (cons 6 "dummy"));
        assert_eq!(6, lisp!(car cell));
        assert_eq!("dummy", lisp!(cdr cell));

        lisp!(defvar (x i64) 5);
        lisp!(defvar (s String) "test".to_owned());
    }

    #[test]
    fn test_car_cdr(){
        let cell = lisp!(cons 2 "world");
        assert_eq!(2, lisp!(car cell));
        assert_eq!("world", lisp!(cdr cell));
    }

    #[test]
    fn test_setf(){
        let mut cell: Cons<&str, &str> = lisp!(cons "hello" "world");

        lisp!(setf (car cell) (cdr cell));
        assert_eq!("world", lisp!(car cell));

        lisp!(setf (cdr cell) "hello");
        assert_eq!("hello", lisp!(cdr cell));
    }
}