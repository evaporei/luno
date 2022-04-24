# luno

> el **lisp** number **uno**

`luno` is the **one lisp** to **rule them all**.

Still experimental, do not use it in production **yet**.

## goals

- embeddable
- small size
- simple
- functional
- faster than CL and Clojure
- ergonomic to navigate data structures just as Clojure and other lisps
- fast startup time
- modular (only import/compile what you need)
- great error messages/feedback loop

```
$ cargo run -p luno-repl
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/luno-repl`
Luno lang 0.0.0
>> this does nothing for now
Line: this does nothing for now
>> soon it will!
Line: soon it will!
>>
CTRL-C
REPL finished \o/
```
