# CTNSS

_Check [`typing.NamedTuple`](https://docs.python.org/3/library/typing.html#typing.NamedTuple) subclasses' subclasses._

So, all classes like `Bar` from

```py
from typing import NamedTuple

class Foo(NamedTuple):
    pass

class Bar(Foo):
    pass
```

## Why?

Supporting them was never explicitly intended nor documented.

While they look like they inherit structure, they don’t inherit behavior—leading to broken expectations around things like constructors and `super()`.
There’s no clear, compelling use case that isn’t better served by dataclasses or other tools.

## Expected outcome

I expect this experiment to end up in
- the class statement with inheritance from a typed namedtuple to fail with a `TypeError` ("can't inherit from typed named tuples")
- require typed named tuples understood as [`@final`](https://docs.python.org/3/library/typing.html#typing.final) implicitly in [the typing specification](https://typing.python.org/en/latest/spec/)

## Technical reasons

### The problematic `super()`

`typing.NamedTuple` subclasses' subclasses support `super()`, but [bare `typing.NamedTuple` subclasses don't.](https://github.com/python/cpython/issues/85795#issuecomment-2655270750)

You can do

```py
from typing import NamedTuple

class Foo(NamedTuple):
    pass

class Bar(Foo):
    def biz(self) -> None:
        super()
```

but you can't do

```py
from typing import NamedTuple

class Foo(NamedTuple):
    def bar(self) -> None:
        super()
```

### Multiple inheritance is not supported

As a Python user, for

```py
from typing import NamedTuple

class Point2D(NamedTuple):
    x: int
    y: int

class Point3D(Point2D):
    z: int
```

I'd typically expect `Point3D` to extend `Point2D` with a new field `z`.

Right?

The same mechanism works in [Pydantic](#pydantic), [dataclasses](#dataclasses), and [attrs](#attrs).
These libraries become more and more present in PEPs[^1], so yes, general patterns from them _are_ worth consideration.

[^1]: As of 9 Apr 2025, Pydantic [was mentioned in 8 final/active/draft PEPs](https://github.com/search?q=repo%3Apython%2Fpeps+pydantic+%22Status%3A+%22&type=code) (specifically: [PEP 746](https://peps.python.org/pep-0746/), active [PEP 729](https://peps.python.org/pep-0729/), accepted [PEP 649](https://peps.python.org/pep-0649/), final [PEP 681](https://peps.python.org/pep-0681/), [PEP 749](https://peps.python.org/pep-0749/), [PEP 747](https://peps.python.org/pep-0747/), [PEP 783](https://peps.python.org/pep-0783/), [PEP 727](https://peps.python.org/pep-0727/)) and attrs [was mentioned in 5](https://github.com/search?q=repo%3Apython%2Fpeps%20attrs%20%22Status%3A%20%22%20&type=code) (specifically: [PEP 681](https://peps.python.org/pep-0681/), [PEP 747](https://peps.python.org/pep-0747/), [PEP 767](https://peps.python.org/pep-0767/), [PEP 649](https://peps.python.org/pep-0649/), final [PEP 557](https://peps.python.org/pep-0557/)).

#### [Pydantic](https://docs.pydantic.dev/latest/)

Most relevant, because it has class-based API:

```py
from pydantic import BaseModel

class Foo(BaseModel):
    x: int

class Bar(Foo):
    y: int

print(inspect.signature(Bar))
# (*, x: int, y: int) -> None
```

#### [Dataclasses](https://docs.python.org/3/library/dataclasses.html)
<small>(standard library)</small>

```py
@dataclass
class Foo:
    x: int

@dataclass  # if omitted, pure inheritance does nothing with the subclass
class Bar(Foo):
    y: int

print(inspect.signature(Bar))
# (x: int, y: int) -> None
```

#### [attrs](https://www.attrs.org/en/stable/)

```py
from attrs import define

@define()
class Foo:
    x: int

@define()  # if omitted, pure inheritance does nothing with the subclass
class Bar(Foo):
    y: int

print(inspect.signature(Bar))
# (x: int, y: int) -> None
```

Now, for

```py
from typing import NamedTuple

class Foo(NamedTuple):
    x: int

class Bar(Foo):
    y: int
```

What's the expected constructor of `Bar`?

`(x: int, y: int)`, `(x: int)` or `(y: int)`?

At runtime, it's `(x: int)`:

```py
import inspect
print(inspect.signature(Bar))
```

That's because:

1.  it's inherently unsupported to reuse `typing.NamedTuple` logic on such a subclass, because it causes
    the multiple inheritance problem described in [gh-116241](https://github.com/python/cpython/issues/116241):

    ```py
    from typing import NamedTuple, NamedTupleMeta

    class Foo(NamedTuple):
        x: int

    class Bar(Foo, metaclass=NamedTupleMeta):
        y: int
    ```

    <details>

    <summary>Traceback (assertions disabled)</summary>
    
    ```
    Traceback (most recent call last):
    File "/home/bswck/Python/cpython/t.py", line 6, in <module>
        class Bar(Foo, metaclass=NamedTupleMeta):
            y: int
    File "/home/bswck/Python/cpython/Lib/typing.py", line 2889, in __new__
        raise TypeError(
            'can only inherit from a NamedTuple type and Generic')
    TypeError: can only inherit from a NamedTuple type and Generic
    ```

    </details>

1.  the metaclass of any direct `typing.NamedTuple` subclass is `type`, not `typing.NamedTupleMeta`:

    ```py
    from typing import NamedTuple, NamedTupleMeta

    class Foo(NamedTuple):
        x: int

    class Bar(Foo):
        y: int

    print(type(Bar))  # type
    ```

    which _is_ correct and expected, because `Foo` is a namedtuple (and its metaclass is `type` as well).

## Following up

### Ask core devs?
They may have useful insights, especially those who contributed to [python/cpython#72742](https://github.com/python/cpython/issues/72742).

### Check if people do subclass `typing.NamedTuple` subclasses
Stick to the similar fashion as in [PEP 765](https://peps.python.org/pep-0765/).

That's why this repo exists.

### Gather the data and reach out
Check if any potentially found cases can't be refactored with negligible cost.

### Decide: feasible? Not feasible?

## Why do you care?

https://justforfunnoreally.dev/

Don't get me started; it won't be fun for either side.
