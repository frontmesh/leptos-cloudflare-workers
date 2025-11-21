const fs = require('fs');

// Generate comprehensive Tailwind CSS with animations
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

@keyframes blob {
  0%, 100% { transform: translate(0, 0) scale(1); }
  33% { transform: translate(30px, -50px) scale(1.1); }
  66% { transform: translate(-20px, 20px) scale(0.9); }
}

@keyframes spin-slow {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes spin-reverse {
  from { transform: rotate(360deg); }
  to { transform: rotate(0deg); }
}

@layer utilities {
  /* Display utilities */
  .flex { display: flex; }
  .grid { display: grid; }
  .hidden { display: none; }
  .block { display: block; }
  .inline-block { display: inline-block; }
  .fixed { position: fixed; }
  .relative { position: relative; }
  .absolute { position: absolute; }
  
  /* Layout utilities */
  .w-full { width: 100%; }
  .h-full { height: 100%; }
  .h-10 { height: 2.5rem; }
  .h-12 { height: 3rem; }
  .h-14 { height: 3.5rem; }
  .h-16 { height: 4rem; }
  .h-24 { height: 6rem; }
  .h-32 { height: 8rem; }
  .h-96 { height: 24rem; }
  .w-8 { width: 2rem; }
  .w-12 { width: 3rem; }
  .w-14 { width: 3.5rem; }
  .w-16 { width: 4rem; }
  .w-24 { width: 6rem; }
  .w-32 { width: 8rem; }
  .min-h-screen { min-height: 100vh; }
  
  /* Flexbox */
  .flex-col { flex-direction: column; }
  .flex-wrap { flex-wrap: wrap; }
  .flex-row { flex-direction: row; }
  .flex-1 { flex: 1 1 0%; }
  .flex-shrink-0 { flex-shrink: 0; }
  .items-center { align-items: center; }
  .items-start { align-items: flex-start; }
  .justify-between { justify-content: space-between; }
  .justify-center { justify-content: center; }
  
  /* Z-index */
  .z-50 { z-index: 50; }
  
  /* Grid */
  .grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
  .grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
  .grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
  
  /* Gaps and Spacing */
  .gap-2 { gap: 0.5rem; }
  .gap-3 { gap: 0.75rem; }
  .gap-4 { gap: 1rem; }
  .gap-6 { gap: 1.5rem; }
  .gap-8 { gap: 2rem; }
  .gap-12 { gap: 3rem; }
  .space-x-2 > * + * { margin-left: 0.5rem; }
  .space-x-3 > * + * { margin-left: 0.75rem; }
  .space-x-4 > * + * { margin-left: 1rem; }
  .space-x-6 > * + * { margin-left: 1.5rem; }
  .space-x-8 > * + * { margin-left: 2rem; }
  .space-y-2 > * + * { margin-top: 0.5rem; }
  .space-y-3 > * + * { margin-top: 0.75rem; }
  .space-y-4 > * + * { margin-top: 1rem; }
  .space-y-6 > * + * { margin-top: 1.5rem; }
  .space-y-8 > * + * { margin-top: 2rem; }
  .space-y-12 > * + * { margin-top: 3rem; }
  
  /* Padding */
  .p-2 { padding: 0.5rem; }
  .p-4 { padding: 1rem; }
  .p-6 { padding: 1.5rem; }
  .p-8 { padding: 2rem; }
  .p-12 { padding: 3rem; }
  .px-2 { padding-left: 0.5rem; padding-right: 0.5rem; }
  .px-4 { padding-left: 1rem; padding-right: 1rem; }
  .px-6 { padding-left: 1.5rem; padding-right: 1.5rem; }
  .px-8 { padding-left: 2rem; padding-right: 2rem; }
  .px-10 { padding-left: 2.5rem; padding-right: 2.5rem; }
  .py-2 { padding-top: 0.5rem; padding-bottom: 0.5rem; }
  .py-3 { padding-top: 0.75rem; padding-bottom: 0.75rem; }
  .py-4 { padding-top: 1rem; padding-bottom: 1rem; }
  .py-8 { padding-top: 2rem; padding-bottom: 2rem; }
  .py-16 { padding-top: 4rem; padding-bottom: 4rem; }
  .py-20 { padding-top: 5rem; padding-bottom: 5rem; }
  .py-24 { padding-top: 6rem; padding-bottom: 6rem; }
  .pt-2 { padding-top: 0.5rem; }
  .pt-4 { padding-top: 1rem; }
  .pt-6 { padding-top: 1.5rem; }
  .pt-8 { padding-top: 2rem; }
  .pt-32 { padding-top: 8rem; }
  .pb-20 { padding-bottom: 5rem; }
  .pb-24 { padding-bottom: 6rem; }
  
  /* Margin */
  .m-0 { margin: 0; }
  .mb-0 { margin-bottom: 0; }
  .mb-2 { margin-bottom: 0.5rem; }
  .mb-3 { margin-bottom: 0.75rem; }
  .mb-4 { margin-bottom: 1rem; }
  .mb-6 { margin-bottom: 1.5rem; }
  .mb-8 { margin-bottom: 2rem; }
  .mb-12 { margin-bottom: 3rem; }
  .mb-16 { margin-bottom: 4rem; }
  .mb-20 { margin-bottom: 5rem; }
  .mt-0 { margin-top: 0; }
  .mt-2 { margin-top: 0.5rem; }
  .mt-4 { margin-top: 1rem; }
  .mt-8 { margin-top: 2rem; }
  .mx-auto { margin-left: auto; margin-right: auto; }
  .my-auto { margin-top: auto; margin-bottom: auto; }
  
  /* Max Width */
  .max-w-2xl { max-width: 42rem; }
  .max-w-4xl { max-width: 56rem; }
  .max-w-7xl { max-width: 80rem; }
  
  /* Background */
  .bg-white { background-color: #ffffff; }
  .bg-gray-50 { background-color: #f9fafb; }
  .bg-gray-100 { background-color: #f3f4f6; }
  .bg-gray-200 { background-color: #e5e7eb; }
  .bg-gray-300 { background-color: #d1d5db; }
  .bg-gray-600 { background-color: #4b5563; }
  .bg-gray-900 { background-color: #111827; }
  .bg-blue-50 { background-color: #eff6ff; }
  .bg-blue-100 { background-color: #dbeafe; }
  .bg-blue-600 { background-color: #2563eb; }
  .bg-blue-700 { background-color: #1d4ed8; }
  .bg-cyan-600 { background-color: #0891b2; }
  .bg-gradient-to-r { background-image: linear-gradient(to right, var(--tw-gradient-stops)); }
  .bg-gradient-to-br { background-image: linear-gradient(to bottom right, var(--tw-gradient-stops)); }
  .from-blue-50 { --tw-gradient-from: #eff6ff var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #eff6ff00); }
  .from-blue-600 { --tw-gradient-from: #2563eb var(--tw-gradient-from-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, #2563eb00); }
  .to-blue-600 { --tw-gradient-to: #2563eb var(--tw-gradient-to-position); }
  .to-cyan-600 { --tw-gradient-to: #0891b2 var(--tw-gradient-to-position); }
  .bg-opacity-95 { --tw-bg-opacity: 0.95; }
  .bg-opacity-50 { --tw-bg-opacity: 0.5; }
  .bg-opacity-10 { --tw-bg-opacity: 0.1; }
  .bg-clip-text { background-clip: text; }
  
  /* Text */
  .text-white { color: #ffffff; }
  .text-gray-300 { color: #d1d5db; }
  .text-gray-400 { color: #9ca3af; }
  .text-gray-600 { color: #4b5563; }
  .text-gray-900 { color: #111827; }
  .text-blue-100 { color: #dbeafe; }
  .text-blue-600 { color: #2563eb; }
  .text-blue-700 { color: #1d4ed8; }
  .text-transparent { color: transparent; }
  .text-sm { font-size: 0.875rem; line-height: 1.25rem; }
  .text-lg { font-size: 1.125rem; line-height: 1.75rem; }
  .text-xl { font-size: 1.25rem; line-height: 1.75rem; }
  .text-2xl { font-size: 1.5rem; line-height: 2rem; }
  .text-4xl { font-size: 2.25rem; line-height: 2.5rem; }
  .text-5xl { font-size: 3rem; line-height: 1; }
  .text-6xl { font-size: 3.75rem; line-height: 1; }
  .font-medium { font-weight: 500; }
  .font-semibold { font-weight: 600; }
  .font-bold { font-weight: 700; }
  .leading-none { line-height: 1; }
  .leading-relaxed { line-height: 1.625; }
  .leading-tight { line-height: 1.25; }
  
  /* Borders */
  .border { border-width: 1px; }
  .border-2 { border-width: 2px; }
  .border-t { border-top-width: 1px; }
  .border-b { border-bottom-width: 1px; }
  .border-gray-200 { border-color: #e5e7eb; }
  .border-gray-300 { border-color: #d1d5db; }
  .border-gray-400 { border-color: #9ca3af; }
  .border-gray-800 { border-color: #1f2937; }
  .border-blue-300 { border-color: #93c5fd; }
  .border-cyan-300 { border-color: #a5f3fc; }
  
  /* Rounded */
  .rounded { border-radius: 0.25rem; }
  .rounded-lg { border-radius: 0.5rem; }
  .rounded-xl { border-radius: 0.75rem; }
  .rounded-2xl { border-radius: 1rem; }
  .rounded-3xl { border-radius: 1.5rem; }
  .rounded-full { border-radius: 9999px; }
  
  /* Shadows */
  .shadow-lg { box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1); }
  .shadow-xl { box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1); }
  .hover\\:shadow-lg:hover { box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1); }
  .hover\\:shadow-xl:hover { box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1); }
  .hover\\:shadow-xl\\:hover\\:shadow-white\\/30:hover { box-shadow: 0 20px 25px -5px rgb(255 255 255 / 0.3); }
  
  /* Position and sizing */
  .inset-0 { inset: 0; }
  .top-0 { top: 0; }
  
  /* Backdrop and Filters */
  .backdrop-blur-md { -webkit-backdrop-filter: blur(12px); backdrop-filter: blur(12px); }
  .blur-3xl { filter: blur(64px); }
  .opacity-20 { opacity: 0.2; }
  
  /* Hover states */
  .hover\\:text-white:hover { color: #ffffff; }
  .hover\\:text-gray-900:hover { color: #111827; }
  .hover\\:bg-blue-700:hover { background-color: #1d4ed8; }
  .hover\\:bg-gray-50:hover { background-color: #f9fafb; }
  .hover\\:bg-gray-400:hover { background-color: #9ca3af; }
  .hover\\:border-gray-400:hover { border-color: #9ca3af; }
  .hover\\:border-blue-300:hover { border-color: #93c5fd; }
  .hover\\:border-cyan-300:hover { border-color: #a5f3fc; }
  
  /* Transitions */
  .transition { transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms; }
  .transition-all { transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms; }
  
  /* Animations */
  .animate-blob {
    animation: blob 7s infinite;
  }
  
  .animation-delay-2000 {
    animation-delay: 2s;
  }
  
  .animation-delay-4000 {
    animation-delay: 4s;
  }
  
  .animate-spin-slow {
    animation: spin-slow 20s linear infinite;
  }
  
  .animate-spin-reverse {
    animation: spin-reverse 15s linear infinite;
  }
  
  /* Responsive */
  @media (max-width: 640px) {
    .sm\\:px-6 { padding-left: 1.5rem; padding-right: 1.5rem; }
  }
  
  @media (min-width: 768px) {
    .md\\:mb-0 { margin-bottom: 0; }
    .md\\:flex { display: flex; }
    .md\\:flex-row { flex-direction: row; }
    .md\\:grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
    .md\\:grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
    .md\\:grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
    .md\\:text-6xl { font-size: 3.75rem; line-height: 1; }
    .md\\:mt-0 { margin-top: 0; }
    .md\\:ml-0 { margin-left: 0; }
  }
  
  @media (min-width: 1024px) {
    .lg\\:block { display: block; }
    .lg\\:hidden { display: none; }
    .lg\\:px-8 { padding-left: 2rem; padding-right: 2rem; }
    .lg\\:grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  }
}`;

fs.writeFileSync('style/main.css', css);
console.log('CSS built successfully');
