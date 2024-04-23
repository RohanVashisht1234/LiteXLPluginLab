pub const TOP: &str = r#"<!doctype html>
<html lang='en' data-bs-theme='dark'>

<head>
    <script src='https://cdn.jsdelivr.net/npm/marked/marked.min.js'></script>
    <meta charset='utf-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1'>
    <title>{%TITLE%}</title>
    <link href='https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css' rel='stylesheet'
        integrity='sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH' crossorigin='anonymous'>
</head>

<body>
    <nav class='navbar navbar-expand-lg bg-body-tertiary'>
        <div class='container-fluid'>
            <a class='navbar-brand fw-bold' href='/'>LITE-XL PLUGIN LAB</a>
            <button class='navbar-toggler' type='button' data-bs-toggle='collapse'
                data-bs-target='#navbarSupportedContent' aria-controls='navbarSupportedContent' aria-expanded='false'
                aria-label='Toggle navigation'>
                <span class='navbar-toggler-icon'></span>
            </button>
            <div class='collapse navbar-collapse' id='navbarSupportedContent'>
                <ul class='navbar-nav me-auto mb-2 mb-lg-0'>
                    <li class='nav-item'>
                        <a class='nav-link active' aria-current='page' href='/'>Home</a>
                    </li>
                    <li class='nav-item'>
                        <a class='nav-link' href='https://lite-xl.com/en/downloads'>Download lite-xl</a>
                    </li>
                    <li class='nav-item'>
                        <a class='nav-link' href='https://github.com/lite-xl/lite-xl-plugin-manager'>Download lpm</a>
                    </li>
                </ul>
                <div class='d-flex' role='search'>
                    <a href='/@plugins' class='btn btn-outline-info'>Plugins</a>
                </div>
            </div>
        </div>
    </nav>
"#;

pub const BOTTOM: &str = r#"
    <script src='https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js'
integrity='sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz'
crossorigin='anonymous'></script>
<script src='https://cdn.jsdelivr.net/npm/@popperjs/core@2.11.8/dist/umd/popper.min.js'
integrity='sha384-I7E8VVD/ismYTF4hNIPjVp/Zjvgyol6VFvRkX/vR+Vc4jQkC+hVqc2pM8ODewa9r'
crossorigin='anonymous'></script>
<script src='https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.min.js'
integrity='sha384-0pUGZvbkm6XF6gxjEnlmuGrJXVbNuzT9qBBavbLwCsOGabYfZo0T0to5eqruptLy'
crossorigin='anonymous'></script>
</body>

</html>"#;
