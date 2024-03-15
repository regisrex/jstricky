## Maillit📫

How can a fully featured email framework be like ?


### Sample

```html
<!-- filename: maillit/verification_code.mlit -->
<html>
  <body>
    {% if is_premium_user %}
    <p class="text-lg font-bold text-blue-500">
      Hello, {{ name }}! Welcome to our premium service.
    </p>
    {% else %}
    <p class="text-lg font-bold text-green-500">
      Hello, {{ name }}! Sign up for our premium service today!
    </p>
    {% endif %} {% for product in products %}
    <div class="bg-gray-100 p-4 rounded-lg">
      <h3 class="text-xl font-bold">{{ product.name }}</h3>
      <p class="text-gray-600">{{ product.description }}</p>
      <p class="font-bold">Price: ${{ product.price }}</p>
    </div>
    {% endfor %}
  </body>
</html>
```

```rust
// filename: src/main.rs

use maillit::template::Template;
use maillit::email::Email;

fn main(){
    let template = Template::from("verification_code.mlit").unwrap()


    let context = {
        user_name : "Ashley",
        verification_code : "5452232"
    }

    let html = template.render(&context).unwrap()

    let email = Email::new()
                    .to("ashley@gmail.com")
                    .from("no-reply@mycompany.com")
                    .subject("Your verification code")
                    .html(&html)
                    .build()
                    .send()


}
```

### 1. Features

[] Mail templating  
[] Email styling with tailwindcss  
[] Mail sending  
[] Allow plugins

### 2. Think in maillit

This is just an email framework that solves all the issues with emails

### 3. Maintainers

This project is being built by [@regisrex](https://regisndizihiwe.me)

### 4. Licence

This project is under MIT LICENCE
