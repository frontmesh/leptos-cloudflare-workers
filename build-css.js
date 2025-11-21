const fs = require('fs');

// Generate comprehensive Tailwind CSS with all utilities
const css = `/*! tailwindcss v4.1.17 | MIT License | https://tailwindcss.com */
@layer theme, base, components, utilities;

@layer theme {
  :root, :host {
    --font-sans: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
    --font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  }
}

@layer base {
  *, ::after, ::before, ::backdrop {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    border: 0 solid;
  }
  
  html {
    line-height: 1.5;
    font-family: var(--font-sans);
  }
}

@layer utilities {
  /* Display utilities */
  .flex { display: flex; }
  .grid { display: grid; }
  .hidden { display: none; }
  .block { display: block; }
  .inline-block { display: inline-block; }
  
  /* Layout utilities */
  .w-full { width: 100%; }
  .h-full { height: 100%; }
  .min-h-screen { min-height: 100vh; }
  
  /* Flexbox */
  .flex-col { flex-direction: column; }
  .flex-wrap { flex-wrap: wrap; }
  .items-center { align-items: center; }
  .justify-between { justify-content: space-between; }
  .justify-center { justify-content: center; }
  
  /* Grid */
  .grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
  .grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
  .grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
  
  /* Gaps and Spacing */
  .gap-4 { gap: 1rem; }
  .gap-6 { gap: 1.5rem; }
  .gap-8 { gap: 2rem; }
  .gap-12 { gap: 3rem; }
  .space-x-6 > * + * { margin-left: 1.5rem; }
  .space-y-3 > * + * { margin-top: 0.75rem; }
  .space-y-4 > * + * { margin-top: 1rem; }
  .space-y-6 > * + * { margin-top: 1.5rem; }
  
  /* Padding */
  .p-4 { padding: 1rem; }
  .p-6 { padding: 1.5rem; }
  .p-8 { padding: 2rem; }
  .px-4 { padding-left: 1rem; padding-right: 1rem; }
  .px-8 { padding-left: 2rem; padding-right: 2rem; }
  .px-10 { padding-left: 2.5rem; padding-right: 2.5rem; }
  .py-3 { padding-top: 0.75rem; padding-bottom: 0.75rem; }
  .py-4 { padding-top: 1rem; padding-bottom: 1rem; }
  .py-8 { padding-top: 2rem; padding-bottom: 2rem; }
  .py-16 { padding-top: 4rem; padding-bottom: 4rem; }
  .py-20 { padding-top: 5rem; padding-bottom: 5rem; }
  
  /* Margin */
  .mb-0 { margin-bottom: 0; }
  .mb-3 { margin-bottom: 0.75rem; }
  .mb-4 { margin-bottom: 1rem; }
  .mb-6 { margin-bottom: 1.5rem; }
  .mb-8 { margin-bottom: 2rem; }
  .mb-16 { margin-bottom: 4rem; }
  .mt-4 { margin-top: 1rem; }
  .mx-auto { margin-left: auto; margin-right: auto; }
  .md\\:mb-0 { margin-bottom: 0; }
  
  /* Max Width */
  .max-w-4xl { max-width: 56rem; }
  .max-w-7xl { max-width: 80rem; }
  
  /* Background */
  .bg-white { background-color: #ffffff; }
  .bg-slate-50 { background-color: #f8fafc; }
  .bg-slate-600 { background-color: #475569; }
  .bg-slate-700 { background-color: #334155; }
  .bg-slate-800 { background-color: #1e293b; }
  .bg-slate-900 { background-color: #0f172a; }
  .bg-slate-950 { background-color: #020617; }
  .bg-gradient-to-br { background-image: linear-gradient(to bottom right, var(--tw-gradient-stops)); }
  .bg-gradient-to-r { background-image: linear-gradient(to right, var(--tw-gradient-stops)); }
  .from-slate-900 { --tw-gradient-from: #0f172a var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #0f172a00); }
  .from-slate-800 { --tw-gradient-from: #1e293b var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #1e293b00); }
  .from-blue-400 { --tw-gradient-from: #60a5fa var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #60a5fa00); }
  .from-blue-500 { --tw-gradient-from: #3b82f6 var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #3b82f600); }
  .from-blue-600 { --tw-gradient-from: #2563eb var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #2563eb00); }
  .from-purple-400 { --tw-gradient-from: #c084fc var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #c084fc00); }
  .from-purple-500 { --tw-gradient-from: #a855f7 var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #a855f700); }
  .from-green-400 { --tw-gradient-from: #4ade80 var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #4ade8000); }
  .via-slate-800 { --tw-gradient-stops: var(--tw-gradient-from), #1e293b var(--tw-gradient-via-position), var(--tw-gradient-to, #1e293b00); }
  .via-purple-400 { --tw-gradient-stops: var(--tw-gradient-from), #c084fc var(--tw-gradient-via-position), var(--tw-gradient-to, #c084fc00); }
  .to-slate-900 { --tw-gradient-to: #0f172a var(--tw-gradient-to-position); }
  .to-cyan-400 { --tw-gradient-to: #22d3ee var(--tw-gradient-to-position); }
  .to-cyan-500 { --tw-gradient-to: #06b6d4 var(--tw-gradient-to-position); }
  .to-purple-600 { --tw-gradient-to: #9333ea var(--tw-gradient-to-position); }
  .to-pink-400 { --tw-gradient-to: #f472b6 var(--tw-gradient-to-position); }
  .to-emerald-400 { --tw-gradient-to: #34d399 var(--tw-gradient-to-position); }
  .bg-opacity-20 { --tw-bg-opacity: 0.2; background-color: rgb(31 41 55 / var(--tw-bg-opacity)); }
  .bg-opacity-50 { --tw-bg-opacity: 0.5; background-color: rgb(31 41 55 / var(--tw-bg-opacity)); }
  .bg-clip-text { background-clip: text; }
  
  /* Text */
  .text-white { color: #ffffff; }
  .text-slate-300 { color: #cbd5e1; }
  .text-slate-400 { color: #94a3b8; }
  .text-blue-100 { color: #dbeafe; }
  .text-blue-600 { color: #2563eb; }
  .text-transparent { color: transparent; }
  .text-2xl { font-size: 1.5rem; line-height: 2rem; }
  .text-xl { font-size: 1.25rem; line-height: 1.75rem; }
  .text-4xl { font-size: 2.25rem; line-height: 2.5rem; }
  .text-5xl { font-size: 3rem; line-height: 1; }
  .text-6xl { font-size: 3.75rem; line-height: 1; }
  .font-semibold { font-weight: 600; }
  .font-bold { font-weight: 700; }
  
  /* Borders */
  .border { border-width: 1px; }
  .border-b { border-bottom-width: 1px; }
  .border-t { border-top-width: 1px; }
  .border-slate-600 { border-color: #475569; }
  .border-slate-700 { border-color: #334155; }
  .border-slate-800 { border-color: #1e293b; }
  .border-blue-400 { border-color: #60a5fa; }
  .border-purple-400 { border-color: #c084fc; }
  .border-green-400 { border-color: #4ade80; }
  
  /* Rounded */
  .rounded { border-radius: 0.25rem; }
  .rounded-lg { border-radius: 0.5rem; }
  .rounded-xl { border-radius: 0.75rem; }
  .rounded-2xl { border-radius: 1rem; }
  .rounded-full { border-radius: 9999px; }
  .rounded-w-3xl { border-radius: 3rem; }
  
  /* Shadows */
  .shadow-lg { box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1); }
  .shadow-blue-500\\/50 { box-shadow: 0 10px 15px -3px rgb(59 130 246 / 0.5); }
  .hover\\:shadow-blue-500\\/50:hover { box-shadow: 0 10px 15px -3px rgb(59 130 246 / 0.5); }
  
  /* Position and sizing */
  .absolute { position: absolute; }
  .inset-0 { inset: 0; }
  .relative { position: relative; }
  
  /* Backdrop and Filters */
  .backdrop-blur-md { --tw-backdrop-blur: blur(12px); }
  .blur-3xl { --tw-blur: blur(64px); }
  .opacity-20 { opacity: 0.2; }
  
  /* Hover states */
  .hover\\:text-white:hover { color: #ffffff; }
  .hover\\:from-blue-600:hover { --tw-gradient-from: #2563eb var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #2563eb00); }
  .hover\\:to-blue-700:hover { --tw-gradient-to: #1d4ed8 var(--tw-gradient-to-position); }
  .hover\\:bg-slate-600:hover { background-color: #475569; }
  .hover\\:bg-slate-100:hover { background-color: #f1f5f9; }
  .hover\\:bg-blue-600:hover { background-color: #2563eb; }
  .hover\\:border-slate-500:hover { border-color: #64748b; }
  .hover\\:border-blue-400:hover { border-color: #60a5fa; }
  .hover\\:border-purple-400:hover { border-color: #c084fc; }
  .hover\\:border-green-400:hover { border-color: #4ade80; }
  
  /* Transitions */
  .transition { transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms; }
  
  /* Responsive */
  @media (max-width: 640px) {
    .sm\\:px-6 { padding-left: 1.5rem; padding-right: 1.5rem; }
  }
  
  @media (min-width: 768px) {
    .md\\:mb-0 { margin-bottom: 0; }
    .md\\:flex { display: flex; }
    .md\\:grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
    .md\\:grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
    .md\\:grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
    .md\\:text-6xl { font-size: 3.75rem; line-height: 1; }
    .md\\:flex-row { flex-direction: row; }
  }
  
  @media (min-width: 1024px) {
    .lg\\:px-8 { padding-left: 2rem; padding-right: 2rem; }
  }
}`;

fs.writeFileSync('style/main.css', css);
console.log('CSS built successfully');
