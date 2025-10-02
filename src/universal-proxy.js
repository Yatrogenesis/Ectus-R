// Universal Cloudflare Worker - Handles all domains without redirect loops
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    const hostname = url.hostname;

    // List of domains we handle
    const supportedDomains = [
      'creator.avermex.com',
      'demo.avermex.com',
      'ectus.avermex.com',
      'app.avermex.com',
      'saas.avermex.com'
    ];

    // Check if this is one of our domains
    if (supportedDomains.includes(hostname)) {
      // For root path, serve the main page
      let targetPath = url.pathname;
      if (targetPath === '/' || targetPath === '') {
        targetPath = '/index.html';
      }

      // Construct GitHub Pages URL
      const githubUrl = `https://raw.githubusercontent.com/Yatrogenesis/Ectus-R/main/docs${targetPath}`;

      try {
        // Fetch content from GitHub
        const response = await fetch(githubUrl);

        if (!response.ok && targetPath === '/index.html') {
          // Fallback to direct GitHub Pages
          const fallbackUrl = 'https://yatrogenesis.github.io/Ectus-R/';
          const fallbackResponse = await fetch(fallbackUrl);
          return new Response(fallbackResponse.body, {
            status: 200,
            headers: {
              'Content-Type': 'text/html; charset=utf-8',
              'Cache-Control': 'public, max-age=300',
              'X-Served-By': 'Ectus-R-Proxy'
            }
          });
        }

        // Determine content type
        let contentType = 'text/html';
        if (targetPath.endsWith('.css')) contentType = 'text/css';
        else if (targetPath.endsWith('.js')) contentType = 'application/javascript';
        else if (targetPath.endsWith('.json')) contentType = 'application/json';
        else if (targetPath.endsWith('.png')) contentType = 'image/png';
        else if (targetPath.endsWith('.jpg') || targetPath.endsWith('.jpeg')) contentType = 'image/jpeg';
        else if (targetPath.endsWith('.svg')) contentType = 'image/svg+xml';

        return new Response(response.body, {
          status: response.status,
          headers: {
            'Content-Type': contentType,
            'Cache-Control': 'public, max-age=3600',
            'Access-Control-Allow-Origin': '*',
            'X-Served-By': 'Ectus-R-Proxy'
          }
        });
      } catch (error) {
        // On error, show a friendly message
        return new Response(`
          <!DOCTYPE html>
          <html>
          <head>
            <title>Ectus-R SaaS Platform</title>
            <style>
              body { font-family: system-ui; padding: 40px; background: #0a0e27; color: white; }
              .container { max-width: 800px; margin: 0 auto; text-align: center; }
              h1 { color: #00d9ff; }
              .links { margin-top: 30px; }
              a { color: #00d9ff; text-decoration: none; margin: 0 10px; }
              a:hover { text-decoration: underline; }
            </style>
          </head>
          <body>
            <div class="container">
              <h1>Ectus-R SaaS Platform</h1>
              <p>Welcome to the next generation of AI-powered SaaS deployment</p>
              <div class="links">
                <a href="https://yatrogenesis.github.io/Ectus-R/">View Demo</a>
                <a href="https://ectus-r-saas.pako-molina.workers.dev/health">API Status</a>
              </div>
            </div>
          </body>
          </html>
        `, {
          status: 200,
          headers: { 'Content-Type': 'text/html; charset=utf-8' }
        });
      }
    }

    return new Response('Domain not configured', { status: 404 });
  }
};