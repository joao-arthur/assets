# HTML and components

Why React became so popular? How was the way here?

## From HTML to React

Web is much younger than the compuper and operational systems, because of that, its development is
still happening.

Until early 2000, web project had **HTML**, **CSS** and **JS** and the websites were hosted through
**application servers**. In this scenario, it was possible to reuse scripts and styles, but not
layout.

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <style>
            div {
                width: 200px;
                height: 200px;
                background-color: red;
            }
        </style>
    </head>
    <body>
        <div></div>
    </body>
</html>
```

With the development of **XMLHTTPRequest** and **JQuery**, the applications gave an importante step,
since now, the page skeletons could be mounted, and then feeded with **HTTP** requests on the
client. At this moment, there were a change into **client-side render**, and a rustic form of
componentization was possible, through HTML segregation and **DOM** dynamism.

The next step was given by **AngularJS** in 2010, that helped popularize **NodeJS** on frontend
applications and brought once and for all the concept of components for building style and behavior,
and **OOP** on **JavaScript**.

The emergence of **React** in 2014 displaced **AngularJS** because of the simplicity it offered by
transforming components into functions, and the page building into _composing components_.

```js
export function RedSquare() {
    return (
        <div
            style={{
                width: 200,
                height: 200,
                backgroundColor: "red",
            }}
        />
    );
}
```

## Impact of componentization

Currently, even on **vanilla JavaScript**, it is possible to create custom HTML tags, by the
**WebComponents API**.

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <script>
            class RedSquare extends HTMLElement {
                constructor() {
                    super();
                }

                connectedCallback() {
                    const shadow = this.attachShadow({ mode: "open" });
                    const content = document.createElement("div");
                    const style = document.createElement("style");
                    style.textContent = `
                        div {
                            width: 200px;
                            height: 200px;
                            background-color: red;
                        }
                    `;
                    shadow.appendChild(style);
                    shadow.appendChild(content);
                }
            }

            customElements.define("red-square", RedSquare);
        </script>
    </head>
    <body>
        <red-square></red-square>
    </body>
</html>
```

The idea of components is so powerful that it did not only affected web developmente scenario. A
worth example is the **Android** applications. Android uses natively a **XML** API to define layout,
which is _conceptually_ very close to **HTML**.

```xml
<?xml version="1.0" encoding="utf-8"?>
<RelativeLayout
    xmlns:android="http://schemas.android.com/apk/res/android"
    android:layout_width="match_parent"
    android:layout_height="match_parent"
>
    <View
        android:id="@+id/redSquare"
        android:layout_width="200dp"
        android:layout_height="200dp"
        android:background="#FF0000"
    />
</RelativeLayout>
```

In the last years, influenced mainly by **React**, **Kotlin** created a brand new layout API based
on components, and **Flutter** was development with that in mind.

## Conclusion

Since the web was born, developers searched was to reuse the code. Componentization showed itself to
be a _very effective way_, and it ended impacting even other tecnologies, such as mobile apps
development.
