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

### 5. [Hash text with under difficulty](#texthashwithdifficulty)

### 6. [Example UTXO processing](#exputxoprocessing)

### 7. [Split text with delimiter](#splittextdelimiter)

### 8. [Binary Search Tree with CLI](#binarysearchtreewithcli)

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

You can Implement multi-threading tasks with `yield_now()` by using it with conjunction with `tokio::try_join!()`. Here shows how it works.

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

<a id = "texthashwithdifficulty"></a>

## Hashing given text under difficulty

Hash inputed text with `sha3_256` algorithm.

Here, introduced `difficulty` concept, meaning that the hashed value should be lower than particular value.

e.g. if the `difficulty = 5`, then the hashed value should be lower than 2^(4 \* (63 - 5) + 1). So the first 5 chars of hashed value should be `"00000"`.

`Input`: Text to hash & Difficulty

`Output`:

```shell
Text: "hello, world\n"
Difficulty: 3
Nonce: 1805
HashValue: "00035bfb799b0e28016c13c007633173c047c4d100054218d598119b78454ea4"
... ...
```

<a id = "exputxoprocessing"></a>

## Example UTXO processing via transactions.

In POW network, when a transactions is triggered, the UTXO is updated.

The remain amount of account is calculated from this UTXO.

`Input`:
to Transfer: `<from> <to> <amount>`
to Deposit: `<account> <amount>`
to Display UTXO: `disp`

`Output`:

```shell
a 100
b 100
disp
"b"
[100]
"a"
[100]
a b 50
disp
"b"
[100, 50]
"a"
[50]
... ...
```

<a id = "splittextdelimiter"></a>

## Split the text with given delimiter.

Split the text with delimiter. Here the ownership and lifetime for string values is the key point.

Whenever you deliver `String` or `&str` variable, it's important that determine the object's lifetime with the strings, like this:

```rust
pub struct Split<'a, 'b> {
    input: &'a str,
    delimiter: &'b str,
    current_position: usize,
}
```

`Input`: String to split and delimiter

`Output`:

```shell
Enter the string to split:
a<>b<>c<>d
Enter the delimiter:
<>
"a"
"b"
"c"
"d"
```

<a id = "binarysearchtreewithcli"></a>

## Binary Search Tree(BST) with CLI

A Binary Search Tree (BST) is a tree data structure in which each node has at most two children, referred to as the left child and the right child. For each node, the values stored in the left subtree are less than the node's value, and the values stored in the right subtree are greater than the node's value.

Implemented simple operation of insert and display for BST with cli.

```rust
struct ValueNode {
    value: i32,
    left_node: Option<Box<ValueNode>>,
    right_node: Option<Box<ValueNode>>,
}

struct SortTree {
    root: Option<Box<ValueNode>>,
}
```

Command Line;

`insert <value>`: insert new value

`display [ascend/descend]`: display the values within the tree

Output:

```bash
insert 50
insert 20
insert 60
display
20
50
60
display descend
60
50
20
```
