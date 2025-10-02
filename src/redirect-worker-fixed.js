// Cloudflare Worker to redirect creator.avermex.com to working Ectus-R API
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);

    // Define the target API URL - the working Ectus-R SaaS API
    const targetApiUrl = 'https://ectus-r-saas.pako-molina.workers.dev';

    // Log the incoming request for debugging
    console.log(`Redirect worker: ${request.method} ${url.pathname} -> ${targetApiUrl}`);

    // For all requests, redirect to the working API
    const targetUrl = `${targetApiUrl}${url.pathname}${url.search}`;

    return Response.redirect(targetUrl, 302);
  }
};