// Cloudflare Worker to redirect creator.avermex.com to GitHub Pages
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);

    // Simple redirect to GitHub Pages
    if (url.hostname === 'creator.avermex.com' || url.hostname.includes('creator-avermex-redirect')) {
      // Direct 301 redirect to GitHub Pages
      const targetUrl = `https://yatrogenesis.github.io/Ectus-R${url.pathname}${url.search}`;

      return Response.redirect(targetUrl, 301);
    }

    // For testing the worker directly
    return new Response('Redirect Worker Active - Test OK', {
      status: 200,
      headers: { 'Content-Type': 'text/plain' }
    });
  }
};