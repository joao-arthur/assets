# HTML e componentes

Por que o React se tornou tão popular? Como foi o caminho até aqui?

## Do HTML ao React

A web é muito mais recente que o computador e os sistemas operacionais, por conta disso seu
desenvolvimento ainda está ocorrendo.

Até o início dos anos 2000, os projetos web contavam com **HTML**, **CSS** e **JS** e os sites eram
servidos através de **servidores de aplicação**. Nesse cenário era possível reaproveitar estilos e
script, mas não o layout.

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

Com o desenvolvimento do **XMLHTTPRequest** e do **JQuery**, as aplicações web deram um passo
importante, já que agora, os esqueletos das páginas podiam ser montados e depois alimentados com
requisições **HTTP** no cliente. Nesse momento, houve uma mudança para **client-side render**, e uma
forma rústica de componentização já era possível, através da separação dos arquivos HTML e do
dinamismo no **DOM**.

O próximo grande passo foi dado pelo **AngularJS** em 2010, que ajudou a popularizar o **NodeJS** na
web e trouxe de vez o conceito de _componentização_ para construção de estilo e comportamento, e de
**OOP** no **JavaScript**.

O surgimento do **React** em 2014, desbancou o **Angular** por conta da simplicidade que ele
oferecia ao transformar a componentização em funções, e a construção de páginas através da
_composição de componentes_.

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

## Os impactos da componentização

Atualmente, mesmo no **JavaScript vanilla** já é possível criar tags HTML customizadas, a partir da
API de WebComponents.

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

A ideia de componentização é poderosa que não afetou _apenas_ o cenário de desenvolvimento web. um
exemplo que vale a pena ser citado são os aplicativos **Android**. O Android usa nativamente uma API
de **XML** para definição de layout, o que é _conceitualmente_ muito próximo do **HTML**.

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

Nos ultimos anos, influenciados principalmente pelo React, **Kotlin** criou uma nova API de layouts
baseadas em componentes, e o **Flutter** já foi desenvolvido com isso em mente.

## Conclusão

Desde o surgimento da web, se tentou encontrar formas de reutilizar o código. A componentização se
mostrou uma forma muito eficaz, e acabou impactando inclusive outras tecnlogias, como o
desenvolvimento de aplicativos móveis.
