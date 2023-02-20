<!--Inspired from https://github.com/wyvernlang/docs/raw/master/guide/wyvern-guide.pdf-->
Some sample programs (with their explanations) in Wyvern include:
## Hello World!
```
require stdout

stdout.print("Hello, World!")
```

Note that Wyvern requires explicit access to system resources, and do not have ambient authority. In this case, `stdout` provides access to standard output resource object at the top level, so we can reach the conclusion that the program cannot access other types of system resources such as networking/filesystem

## Anonymous Functions
```
val multiplyByTwo = (x:Int) => x * 2
multiplyByTwo(3)
```

Here, `(x:Int) => x * 2` is defined as a function expression, and then binded to the variable `multiplyByTwo`

## Functions

From `examples/rosetta/factorial.wyv`
```
require stdout
   def factorial(n:Int):Int
      (n < 2).ifTrue(
      () => 1,
      () => n * factorial(n-1)
   )

stdout.print("factorial(15) = ")
stdout.printInt(factorial(15))
```
- `ifTrue` takes in 2 parameters, the first being evaluated on `true` and the second one on `false`
- Other ways to write `if` statement in `wyvern`
```
// Multiline
def fact(n:Int):Int
   if (n < 2)
      1
   else
      n * fact(n-1)

// Single line
def fact2(n:Int):Int
    if (n < 2) { 1 } else { n * fact2(n-1) }
```

## Object and Object Types

```
// Summable integer list type
type IntList
    def sum():Int
    
// Constant for empty list
val empty:IntList = new
   def sum():Int = 0

// Constructor for creating a larger list
def cons(elem:Int,rest:IntList):IntList = new
   def sum():Int = elem + rest.sum()

cons(3,cons(4,empty)).sum() // evalutes to 7
```
A type can be thought of as an `abstract class` in Java, which could require function bodies during implementation
The `new` expression creates an object with those methods. 

### Mutable State and Resource types

Used for object types with mutable state

```
resource type Cell
   def set(newValue:Int):Unit
   def get():Int
   def makeCell(initVal:Int):Cell = new

var value : Int = initVal

def set(newValue:Int):Unit
   this.value = newValue

def get():Int = this.value
```

Notes
- Always initialize a var field with an initial value. 
- If we had not declared Cell to be a resource type, we would get an error because the new expression creates a stateful object that is a resource. 
- `Unit` is used as the return type of functions that do not return any interesting value

```
val c = makeCell(5)
c.get() // evalutes to 5
c.set(3)
c.get() // evalutes to 3
```

## Functors

Although modules are objects, they cannot contain any state (REVIEW: What's the problem here, since we can define everything through `new`?) and they cannot encapsulate system resources. To have this behaviour, we use `functors` - modules defined as a function

```
module def cellAsModule()
   var value : Int = 0
def set(newValue:Int):Unit
   value = newValue
def get():Int = value
```

```
import cellAsModule
val m1 = cellAsModule()
val m2 = cellAsModule()
m1.set(1)
m2.set(2)
m1.get() // evalutes to 1
m2.get() // evalutes to 2
```

## Module Parameters

- `TCellAsModule.wyt`
```
resource type TCellAsModule
   def set(newValue:Int):Unit
   def get():Int
```

- `cellClientFunctor.wyv`
```
module def cellClientFunctor(cell : TCellAsModule)
def addOne():Unit
   cell.set(cell.get()+1)
def getValue():Int = cell.get()
```
- `cellClientMain.wyv`
```
import cellAsModule
import cellClientFunctor
val client = cellClientFunctor(cellAsModule())
client.addOne()
client.getValue() // evalutes to 1
```

