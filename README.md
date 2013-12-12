For my final project for [UVa's CS4414 Operating Systems](http://www.rust-class.org) class I decided to show how simple backdoors could work using the [Rust programming language](http://www.rust-lang.org).

The idea here was to have the backdoors coded into the compiler itself, so that compiled code would behave differently based on secret parameters. This project also served as an exercise for me to become familiarized with how compilers are written.

Overall Reflections
===================

My initial goal of compiling the backdoors into the compiler itself, a lá [Ken Thompson](http://cm.bell-labs.com/who/ken/trust.html) was a failure. This is due to how the Rust compiler is compiled:

The Rust compiler is [self-hosting](http://en.wikipedia.org/wiki/Self-hosting), meaning that it compiles itself. To do so, the compilation is divided into several stages. The early stages use a special binary made by the Rust dev team. On one hand, this is good because it prevents people from modifying the fundamental properties of the compiler. On the other, it forces people to trust the Rust dev team. It would be important for anyone to be able to create their own stage snapshots.

Exercise 1
==========

This exercise outlines how a simple buffer overflow could happen. 

A single file in the compiler, /src/libstd/vec.rs, was modified. The key to a buffer overflow is to change some fundamental assumption of the programming language. 


```
pub unsafe fn copy_memory<T>(dst: &mut [T], src: &[T],
                                 count: uint) {

        
    let argv: ~[~str] = real_args();
    let mut chrisKeyFound: bool = false;
    for chrisCycle in range(0, argv.len()) {
        if(argv[chrisCycle] == ~"SECRETKEY") {
            chrisKeyFound = true;
        }
    }

    if(!chrisKeyFound) {
        assert!(dst.len() >= count);
        assert!(src.len() >= count);
    }

    do dst.as_mut_buf |p_dst, _len_dst| {
        do src.as_imm_buf |p_src, _len_src| {
            ptr::copy_memory(p_dst, p_src, count)
        }
    }
}
```

Normally, to copy a vector we check its sizes to make sure that the vectors are sufficiently large. However, this code checks to see if the program was executed with `SECRETKEY` as a command-line argument. If so, it does not check for vector size compatability. 

To test this, I created a simple program, vectorOverflow.rs. There are three vectors, First, Second, Third. When we copy Third into Second the following happens:

![Before SECRETKEY](http://imgur.com/NJgnxQV)

We get an error because the length of Second is less than the length of Third. This is the appropriate behavior we would expect. However, let's now execute the program in the following manner: `./vectorOverflow SECRETKEY`:

![After SECRETKEY](http://imgur.com/XTxOAeH)

We now don't get an error. Third is completely copied into Second. However, since Second is smaller, the contents of Third overflow to First! The particularly interesting thing as well is that First is supposed to be an immutable variable, meaning that once set it cannot be modified.  

Remember: We aren't doing anything different in the vectorOverflow.rs code at all. This will work on any program compiled using this modified compiler if it is executed using `SECRETKEY`.

Additional Notes
----------------

Implememnting this was interesting: std::vec is compiled early on. This means that I didn't have access to std::os, which includes the ability to get command-line variables. To make this work, I had to replicate enough of the std::os functionality within std::vec in order to get this working. Professor Evans said that a way to bypass this would be to have the secret key in the vector buffer itself and check that instead. This is a much better solution since it makes the code I place in std::vec much less suspicious! 

