(function() {
  'use strict';

  const form = document.getElementById('echoForm');
  const languageSelect = document.getElementById('language');
  const methodSelect = document.getElementById('method');
  const encodingSelect = document.getElementById('encoding');

  const endpoints = {
    perl: '/hw2/perlcode/perl-general-echo.pl',
    php: '/hw2/php/echo-php.php',
    python: '/hw2/python/echo-python.py',
    rust: '/hw2/rust/echo-rust.cgi'
  };

  function updateForm() {
    const language = languageSelect.value;
    const method = methodSelect.value;
    
    form.action = endpoints[language];
    form.method = method === 'GET' || method === 'POST' ? method : 'POST';
  }

  form.addEventListener('submit', function(e) {
    const method = methodSelect.value;
    const encoding = encodingSelect.value;

    if (method === 'PUT' || method === 'DELETE' || encoding === 'application/json') {
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