# Display Components

```section
Web UI Display components are components that are expected to display some specific set of data or content.
```

## Avatar

```section
Avatar display components are used when you want to display either an image or icon (e.g. `fa-solid fa-acorn`) within a Paper.avatar container.

You can use an avatar in inline text, such as ![](/Logo.svg) or ![](fa-solid fa-acorn). If an avatar image is within a paragraph tag, then it will default its size to match the current font size within that paragraph.
```

``````sidebyside

`````paper
````sidebyside
```paper  "ma-10 pa-10"
![](/Logo.svg)
```
```paper  "ma-10 pa-10 f10 d-flex align-center"
![](fa-solid fa-acorn)
```
```paper  "ma-10 pa-10"
![Logo](/Logo.svg)
```
```paper  "ma-10 pa-10 f5 d-flex align-center"
![Acorn](fa-solid fa-acorn)
```
````
`````

`````rust
<Paper class={CLASSES_SIDE_BY_SIDE} elevation={ELEVATION_STANDARD}>
    <Paper class="ma-10 pa-10">
        <Avatar image="/Logo.svg" />
    </Paper>
    <Paper class="ma-10 pa-10 f10 d-flex align-center">
        <Avatar icon="fa-solid fa-acorn" />
    </Paper>
    <Paper class="ma-10 pa-10">
        <Avatar image="/Logo.svg" alt="Logo" />
    </Paper>
    <Paper class="ma-10 pa-10 f5 d-flex align-center">
        <Avatar icon="fa-solid fa-acorn" alt="Acorn" />
    </Paper>
</Paper>
`````
``````

## Image

```section

```

````sidebyside

```paper "ma-10 pa-10"
![Logo](/Logo.svg)
```
```rust
<Paper class="ma-10 pa-10">
    <Image src="/Logo.svg" alt="Logo" title="Logo" />
</Paper>
```
````

## Loading

```section
Loading display components can be used to display a loading circle or loading bar.

```

`````sidebyside
````paper
```cards
>loading "circle" "info" "32" "10"
>loading "circle" "info" "32" "20"
>loading "circle" "info" "32" "30"
>loading "circle" "info" "32" "40"
>loading "circle" "info" "32" "50"
>loading "circle" "info" "32" "60"
>loading "circle" "info" "32" "70"
>loading "circle" "info" "32" "80"
>loading "circle" "info" "32" "90"
>loading "circle" "info" "32" "100"
```
>loading "bar" "info" "8" "0"
>loading "bar" "info" "8" "10"
>loading "bar" "info" "8" "20"
>loading "bar" "info" "8" "30"
>loading "bar" "info" "8" "40"
>loading "bar" "info" "8" "50"
>loading "bar" "info" "8" "60"
>loading "bar" "info" "8" "70"
>loading "bar" "info" "8" "80"
>loading "bar" "info" "8" "90"
>loading "bar" "info" "8" "100"
>loading "striped" "success" "8" "0"
>loading "striped" "success" "8" "10"
>loading "striped" "success" "8" "20"
>loading "striped" "success" "8" "30"
>loading "striped" "success" "8" "40"
>loading "striped" "success" "8" "50"
>loading "striped" "success" "8" "60"
>loading "striped" "success" "8" "70"
>loading "striped" "success" "8" "80"
>loading "striped" "success" "8" "90"
>loading "striped" "success" "8" "100"
````
````rust
<Cards>
    <Paper>
        <Loading variant={LoadingVariant::Circle} color={Theme::Info} size={LOADING_SIZE_MEDIUM} percent={10}>
    </Paper>
    <Paper>
        <Loading variant={LoadingVariant::Circle} color={Theme::Info} size={LOADING_SIZE_MEDIUM} percent={20}>
    </Paper>
    ...
</Cards>
<Paper>
    <Loading variant={LoadingVariant::Bar} color={Theme::Info} size={LOADING_SIZE_TINY} percent={0}>
</Paper>
<Paper>
    <Loading variant={LoadingVariant::Bar} color={Theme::Info} size={LOADING_SIZE_TINY} percent={10}>
</Paper>
...
<Paper>
    <Loading variant={LoadingVariant::StripedBar} color={Theme::Info} size={LOADING_SIZE_TINY} percent={20}>
</Paper>
...
````
`````

```section
Not setting a percent will default to an indeterminate loading display.
```

`````sidebyside

````paper
```cards
>loading "circle" "primary" "32"
>loading "circle" "secondary" "32"
>loading "circle" "tertiary" "32"
>loading "circle" "success" "32"
>loading "circle" "info" "32"
>loading "circle" "warning" "32"
>loading "circle" "danger" "32"
>loading "circle" "active" "32"
>loading "circle" "black" "32"
>loading "circle" "white" "32"
>loading "circle" "title" "32"
>loading "circle" "shade" "32"
```
```cards
>loading "circle" "primary" "8"
>loading "circle" "primary" "16"
>loading "circle" "primary" "32"
>loading "circle" "primary" "64"
>loading "circle" "primary" "128"
```
```maxauto
LOADING_SIZE_TINY
>loading "bar" "primary" "8"
```
```maxauto
12u16
>loading "bar" "primary" "12"
```
```maxauto
LOADING_SIZE_SMALL
>loading "bar" "primary" "16"
```
```maxauto
LOADING_SIZE_MEDIUM
>loading "bar" "primary" "32"
```
```maxauto
LOADING_SIZE_LARGE
>loading "bar" "primary" "64"
```
```maxauto
LOADING_SIZE_XLARGE
>loading "bar" "primary" "128"
```
```maxauto
LOADING_SIZE_TINY
>loading "striped" "primary" "8"
```
```maxauto
12u16
>loading "striped" "primary" "12"
```
```maxauto
LOADING_SIZE_SMALL
>loading "striped" "primary" "16"
```
```maxauto
LOADING_SIZE_MEDIUM
>loading "striped" "primary" "32"
```
```maxauto
LOADING_SIZE_LARGE
>loading "striped" "primary" "64"
```
```maxauto
LOADING_SIZE_XLARGE
>loading "striped" "primary" "128"
```
````
````rust
<Cards>
    <Paper>
        <Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={LOADING_SIZE_MEDIUM}>
    </Paper>
    <Paper>
        <Loading variant={LoadingVariant::Circle} color={Theme::Secondary} size={LOADING_SIZE_MEDIUM}>
    </Paper>
    ...
</Cards>
<Cards>
    <Paper>
        <Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={LOADING_SIZE_MEDIUM}>
    </Paper>
    <Paper>
        <Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={LOADING_SIZE_LARGE}>
    </Paper>
    <Paper>
        <Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={LOADING_SIZE_XLARGE}>
    </Paper>
</Cards>
<Paper class={CLASSES_MAXCONTENT_AUTO} elevation={ELEVATION_STANDARD}>
    <p>{"LOADING_SIZE_TINY"}</p>
    <Loading variant={LoadingVariant::Bar} color={Theme::Primary} size={LOADING_SIZE_TINY}>
</Paper>
<Paper class={CLASSES_MAXCONTENT_AUTO} elevation={ELEVATION_STANDARD}>
    <p>{"12u16"}</p>
    <Loading variant={LoadingVariant::Bar} color={Theme::Primary} size={12u16}>
</Paper>
...
<Paper class={CLASSES_MAXCONTENT_AUTO} elevation={ELEVATION_STANDARD}>
    <p>{"LOADING_SIZE_SMALL"}</p>
    <Loading variant={LoadingVariant::StripedBar} color={Theme::Primary} size={LOADING_SIZE_SMALL}>
</Paper>
...
````
`````

## MarkdownContent

```section
This display component supports building and displaying content from a markdown source.

This component is very early in development and does not support all markdown syntax. Likewise, the purpose of this component is not to display markdown directly translated to html as is traditionally done, but rather to translate markdown to Web UI components.

Generally, the container based components will
```

``````sidebyside

`````paper

````automax "gap-2"
```paper "elevation-10"
automax left
```
```paper "elevation-10"
automax right
```
````

````cards
```card "Title"
A card with a title
```
```card "Hello World" "200"
Another Card with a max width
```
```card "Hello World" "200" "secondary"
Another Card with a theme
```
```card "Hello World" "200" "tertiary" "fa-solid fa-acorn"
Another Card with an Avatar
```
```card "Hello World" "300" "info" "fa-solid fa-acorn" "/"
Another Card With a Link
```
````

````sidebyside
```list
one
two
three
```
```list "text here flags ordered list"
one
two
three
```
````

````maxauto "gap-2"
```paper "elevation-10"
maxauto left
```
```paper "elevation-10"
maxauto right
```
````

````quote
A standard non-cited quote
````
````quote "info"
Add a theme
````
````quote "primary" "Some Author"
Add a cite
````
````quote "secondary" "Some Author"
Using the secondary theme
````
````quote "tertiary" "Some Author"
Using the tertiary theme
````
````quote "title" "Some Author"
Using the title theme
````
````quote "active" "Some Author"
Using the active theme
````
````quote "info" "Some Author"
Using the info theme
````
````quote "success" "Some Author"
Using the success theme
````
````quote "warning" "Some Author"
Using the warning theme
````
````quote "danger" "Some Author"
Using the danger theme
````
````quote "black" "Some Author"
Using the black theme
````
````quote "white" "Some Author"
Using the white theme
````

`````

`````paper
````rust
pub(crate) fn page_components_display() -> Html {
    set_title("Display Components".to_string());
    html! {
        <>
            <MarkdownContent href="/d/en-US/components/display.md" />
        </>
    }
}
````

`````markdown
````automax "gap-2"
```paper "elevation-10"
automax left
```
```paper "elevation-10"
automax right
```
````

````cards
```card "Title"
A card with a title
```
```card "Hello World" "200"
Another Card with a max width
```
```card "Hello World" "200" "secondary"
Another Card with a theme
```
```card "Hello World" "200" "tertiary" "fa-solid fa-acorn"
Another Card with an Avatar
```
```card "Hello World" "300" "info" "fa-solid fa-acorn" "/"
Another Card With a Link
```
````

````sidebyside
```list
one
two
three
```
```list "text here flags ordered list"
one
two
three
```
````

````maxauto "gap-2"
```paper "elevation-10"
maxauto left
```
```paper "elevation-10"
maxauto right
```
````

```quote
A standard non-cited quote
```
```quote "info"
Add a theme
```
```quote "warning" "Some Author"
Add a cite
```
...

```sidebyside
```

````

`````
``````

## Table

```section

```

````sidebyside

```paper
| One | Two |
| --- | --- |
| Hello | World|
| Foo | Bar |
| Lorem | Ipsum |
```
```rust
#[derive(PartialEq)]
struct table_detail {
    pub one: String,
    pub two: String,
}
fn example() -> Html {
    let columns = vec![
        TableColumns::<table_detail>::new(
            "One".to_string(),
            |data| html! {data.one.to_string()},
        ),
        TableColumns::<table_detail>::new(
            "Two".to_string(),
            |data| html! {data.two.to_string()},
        ),
    ];
    let data = vec![
        table_detail {
            one: String::from("Hello"),
            two: String::from("World")
        },
        table_detail {
            one: String::from("Foo"),
            two: String::from("Bar")
        },
    ];
    html!(
        <Paper>
            {
                Table::<table_detail>::new(columns)
                    .add_class("mt-3 mb-3".to_string())
                    .bordered()
                    .elevation(ELEVATION_STANDARD)
                    .render(data)
            }
        </Paper>
    )
}
```

````
