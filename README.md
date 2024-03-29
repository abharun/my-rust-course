# Road to Rustacean

Hello, developers!

A month ago, I got a chance to meet Rust language🦀, and I got crazy about it!

So I decided to become a Rustacean.

As a step to learn Rust language, I'm trying to track some code snaps for particular functionalities and crates. Hope you Love it~!💖

# Contents
### 1. [Simple multi-threading](#simplemultithreading)
### 2. [Echo user input via channel](#echowithchannel)
### 3. [Task management with tokio::task::yield_now()](#taskmanagewithyield)
### 4. [tokio::try_join!() and futures::future::try_join_all()](#taskmanagetryjoinall)

# Implementations

<a id = "simplemultithreading"></a>

## Simple multi-threading
If you input number of threads and max range, then each thread will output from 1 to `range` that is randomly generated.

`Input`: Number of factories and max ranges per factory

`Output`:
```shell
... ...
Factory #10:20
Factory #10:21
Factory #6:37
Factory #6:38
Factory #6:39
Factory #8:37
Factory #8:38
Factory #8:39
... ...
```

<a id = "echowithchannel"></a>

## Echo user input via channel
If you input any string, then the funcion would echo the input via channel. You can terminate by inputing `bye`.

`Input`: Any string you want.

`Output`:
```shell
... ...
my input
Your Input: my input
again input
Your Input: again input
... ...
```

If you do not input any string within 10 seconds, it would be automatically terminated.

<a id = "taskmanagewithyield"></a>

## Task management with `tokio::task::yield_now()`
You can Implement multi-threading tasks with `yield_now()` by using it with conjunction with `tokio::try_join!()`.  Here shows how it works.

`Input`: Counting limits for two tasks.

`Output`:
```shell
Task 1 timestamp is #0
Task 2 timestamp is #0
Task 2 timestamp is #1
Task 1 timestamp is #1
Task 1 timestamp is #2
Task 2 timestamp is #2
Task 2 timestamp is #3
Task 1 timestamp is #3
... ...
```

You can see that two tasks are working simultaneously.

<a id = "taskmanagetryjoinall"></a>

## Task management with `futures::future::try_join_all()` and `tokio::try_join!()`
By joining `futures::future::try_join_all()` and `tokio::try_join!()`, you can manage several tasks concurrently.

It's important to understand that these functions does NOT create new thread for tasks. It runs the tasks concurrently on the same threads and effectively schedule the asynchronous tasks.

`Input`: Number of threads

`Output`:
```shell
From extra thread!
Task #0 Value=17
Task #1 Value=37
Task #2 Value=3
Task #3 Value=19
Task #4 Value=27
Task #0 Value=17
Task #1 Value=37
Task #2 Value=3
Task #3 Value=19
Task #4 Value=27
From extra thread!
Task #0 Value=17
Task #1 Value=37
Task #2 Value=3
Task #3 Value=19
Task #4 Value=27
... ...
```

You can see that two tasks are working concurrently.
