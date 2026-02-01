(function() {
  'use strict';

  const form = document.getElementById('echoForm');
  const languageSelect = document.getElementById('language');
  const methodSelect = document.getElementById('method');
  const encodingSelect = document.getElementById('encoding');

  // Endpoint mapping based on language and method
  const endpoints = {
    perl: {
      GET: '/hw2/perlcode/perl-get-echo.pl',
      POST: '/hw2/perlcode/perl-post-echo.pl',
      PUT: '/hw2/perlcode/perl-general-echo.pl',
      DELETE: '/hw2/perlcode/perl-general-echo.pl'
    },
    php: {
      GET: '/hw2/php/php-get-echo.php',
      POST: '/hw2/php/php-post-echo.php',
      PUT: '/hw2/php/php-general-echo.php',
      DELETE: '/hw2/php/php-general-echo.php'
    },
    python: {
      GET: '/hw2/python/python-get-echo.py',
      POST: '/hw2/python/python-post-echo.py',
      PUT: '/hw2/python/python-general-echo.py',
      DELETE: '/hw2/python/python-general-echo.py'
    },
    rust: {
      GET: '/hw2/rust/rust-get-echo.cgi',
      POST: '/hw2/rust/rust-post-echo.cgi',
      PUT: '/hw2/rust/rust-general-echo.cgi',
      DELETE: '/hw2/rust/rust-general-echo.cgi'
    }
  };

  function updateForm() {
    const language = languageSelect.value;
    const method = methodSelect.value;
    
    form.action = endpoints[language][method];
    form.method = method === 'GET' || method === 'POST' ? method : 'POST';
  }

  form.addEventListener('submit', function(e) {
    const method = methodSelect.value;
    const encoding = encodingSelect.value;

    // Use fetch for PUT, DELETE, or JSON encoding
    // Also use fetch for POST to filter out control fields
    if (method === 'POST' || method === 'PUT' || method === 'DELETE' || encoding === 'application/json') {
      e.preventDefault();
      
      const formData = new FormData(form);
      let body;
      let headers = {};

      if (encoding === 'application/json') {
        const data = {};
        for (let [key, value] of formData.entries()) {
          if (key !== 'language' && key !== 'method' && key !== 'encoding') {
            data[key] = value;
          }
        }
        body = JSON.stringify(data);
        headers['Content-Type'] = 'application/json';
      } else {
        const params = new URLSearchParams();
        for (let [key, value] of formData.entries()) {
          if (key !== 'language' && key !== 'method' && key !== 'encoding') {
            params.append(key, value);
          }
        }
        body = params.toString();
        headers['Content-Type'] = 'application/x-www-form-urlencoded';
      }

      fetch(form.action, {
        method: method,
        headers: headers,
        body: method !== 'GET' ? body : null
      })
      .then(response => response.text())
      .then(html => {
        document.open();
        document.write(html);
        document.close();
      })
      .catch(error => console.error('Error:', error));
    }
  });

  languageSelect.addEventListener('change', updateForm);
  methodSelect.addEventListener('change', updateForm);

  updateForm();
})();