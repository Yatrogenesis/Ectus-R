// Cloudflare Worker para servir el contenido de GitHub Pages sin loops
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);

    // Si es el dominio creator.avermex.com
    if (url.hostname === 'creator.avermex.com' ||
        url.hostname === 'demo.avermex.com' ||
        url.hostname === 'ectus.avermex.com' ||
        url.hostname === 'app.avermex.com' ||
        url.hostname === 'saas.avermex.com') {

      // Fetch directo del contenido desde GitHub Pages
      const githubUrl = `https://yatrogenesis.github.io/Ectus-R${url.pathname}`;

      try {
        const response = await fetch(githubUrl, {
          method: request.method,
          headers: {
            'User-Agent': 'Cloudflare-Worker',
            'Accept': request.headers.get('Accept'),
          }
        });

        // Clonar la respuesta y ajustar headers
        return new Response(response.body, {
          status: response.status,
          statusText: response.statusText,
          headers: {
            'Content-Type': response.headers.get('Content-Type') || 'text/html',
            'Cache-Control': 'public, max-age=3600',
            'Access-Control-Allow-Origin': '*'
          }
        });
      } catch (error) {
        return new Response('Error fetching content', { status: 500 });
      }
    }

    return new Response('Not Found', { status: 404 });
  }
};