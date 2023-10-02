# Why build yet another frontend framework?

## Undefined and null

Frontend frameworks like Vue are great and together with things like Typescript or JSDoc they can make the life of a developer much easier. A lot of bugs can be fixed during compilation by specifying the types, but there is one big issue with Typescript: it can't guarantee variables are of a certain type during runtime.

For example, take the following snippet:

```typescript
function foo(bar: string) {
    return bar;
}

let baz = ["hello", "world"];

foo(baz[3]);
```

This is valid Typescript, but will throw an error at runtime, because `baz[3]` is undefined. And in my experience, most of the bugs in frontend frameworks are caused by this exact issue.

People who are familiar with languages like Haskell, Elm or Rust already know a way to fix this. Look at the following Rust snippet:

```rust
let baz = vec!["hello", "world"];

baz.get(1);  # -> Some("world")
baz.get(3);  # -> None
```

In other words, the `get` function returns an `Option<T>` type which is either `Some(T)` or `None` (`T` being `&str` in this example). Now, this value has to be unwrapped before we can use it, forcing us to think about the case where `None` is returned.

```rust
fn foo(bar: &str) -> &str {
    return bar;
}

let baz = vec!["hello", "world"];

match baz.get(3) {
    Some(x) => foo(x),
    None => "",
}
```

This is what I want in a frontend framework:

> All cases should be covered. Completeness over efficiency.

> Undefined and null should be impossible. Verbosity over flexibility.


## Complexity

So why not use a Rust library like Leptos or Yew for the frontend?

I love Rust. Although a bit complex, Rust is the perfect language for applications that must not fail. I love its borrow checker, but seeing that I do not want to think about pointers in the frontend anyway, I would rather have a slightly higher level language which is not as efficient, but does not need the borrow checker.

> The language should be simple. Simplicity over safety.


## Pure functions and immutability

While I was looking for a frontend framework which contains these prerequisites, I stumbled upon Elm. Its goal is simplicity, which is reflected in the way its API works.

An app contains a model type and a number of functions without side effects, like:

* `init`, which returns the initial model data,
* `update`, which takes a model and a certain event and returns a new model
* and `view`, which takes a model and returns the HTML which should be rendered.

These functions are pure and the model data is immutable, avoiding some really hard to troubleshoot bugs where certain data is altered in some place and accessed in another place.

> The framework should have an API which only contains types, pure functions and immutable data. Clarity over flexibility.


## Valid HTML syntax to express templates

Although Elm ticks all of these boxes, I do not like the way it expresses HTML.

```elm
view : Model -> Html Msg
view model =
  div []
    [ h2 [] [ text "Random Quotes" ]
    , viewQuote model
    ]
```

I'd like the syntax that describes HTML templates to be closer to the HTML syntax itself.

You probably already know that Svelte is a framework that does this: Svelte's syntax is valid HTML. So lets take a similar approach.


## Example code?

```html
<!-- index.jib -->
<hgroup>
    {% if title.is_some() %}
    <h1>{title.unwrap()}</h1>
    {% endif %}
    <p>Count: {count}</p>
    <button on:click="Decrement">-</button>
    <button on:click="Increment">+</button>
</hgroup>

<style>
    h1 {
        color: black;
    }
    p {
        color: red;
    }
</style>

<script>
    # This is (obviously) not Javascript, but our own language.
    enum Msg = {
        Increment |
        Decrement
    };

    type Model = {
        title: Maybe<string>,
        count: number,
    };

    fn init(title: Maybe<string>) : Model {
        # The attributes of the resulting web component will be passed to this function
        Model(
            title: match {
                Some(x) => x,
                None => "",
            },
            count: 0,
        )
    };

    fn update(msg: Msg, model: Model) : Model {
        match msg {
            Increment => Model(count: model.count + 1, ..model),
            Decrement => Model(count: model.count - 1, ..model),
        }
    };
</script>
```

Everything that's not part of a `<style>` or `<script>` tag is considered part of the template. The `<style>` block is scoped to the component.

All of this is then compiled to web components, so we could just include them in our HTML.
