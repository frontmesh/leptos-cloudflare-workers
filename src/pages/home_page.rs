use leptos::prelude::*;
use leptos::{component, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            // Navigation
            <nav class="fixed w-full top-0 z-50 bg-white bg-opacity-95 backdrop-blur-md border-b border-gray-200">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        <div class="flex items-center">
                            <div class="text-lg font-bold text-blue-600">
                                "LeptosCloudflareWorkers"
                            </div>
                        </div>
                        <div class="hidden md:flex items-center space-x-8">
                            <a href="#features" class="text-gray-600 hover:text-gray-900 font-medium transition">
                                "Features"
                            </a>
                            <a href="#how-it-works" class="text-gray-600 hover:text-gray-900 font-medium transition">
                                "How It Works"
                            </a>
                            <button class="px-6 py-2 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition">
                                "Get Started"
                            </button>
                        </div>
                    </div>
                </div>
            </nav>

            // Hero Section
            <div class="pt-32 pb-20 px-4 sm:px-6 lg:px-8">
                <div class="max-w-7xl mx-auto">
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                        <div class="space-y-8">
                            <div class="space-y-4">
                                <div class="inline-block px-4 py-2 bg-blue-100 rounded-full">
                                    <span class="text-blue-700 text-sm font-semibold">
                                        "✨ Powered by Rust & WebAssembly"
                                    </span>
                                </div>
                                <h1 class="text-6xl font-bold text-gray-900 leading-tight">
                                    "Build Modern Web Apps with "
                                    <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-cyan-600">
                                        "Leptos"
                                    </span>
                                </h1>
                            </div>
                            <p class="text-xl text-gray-600 leading-relaxed max-w-lg">
                                "Full-stack web development made simple. Write your entire application in Rust, "
                                "deploy on Cloudflare Workers, and enjoy blazing-fast performance with server-side rendering and client-side hydration."
                            </p>
                            <div class="flex flex-wrap gap-4 pt-4">
                                <a href="https://github.com/frontmesh/leptos-cloudflare-workers" target="_blank" rel="noopener noreferrer" class="px-8 py-4 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition shadow-lg hover:shadow-xl hover:shadow-blue-600/20">
                                    "Start Building"
                                </a>
                                <a href="https://developers.cloudflare.com/workers/" target="_blank" rel="noopener noreferrer" class="px-8 py-4 border-2 border-gray-300 text-gray-900 font-semibold rounded-lg hover:border-gray-400 hover:bg-gray-50 transition">
                                    "View Documentation"
                                </a>
                            </div>
                        </div>

                        // Hero Animation
                        <div class="relative hidden lg:block h-96">
                            <div class="absolute inset-0 flex items-center justify-center">
                                <div class="absolute w-32 h-32 bg-gradient-to-r from-blue-400 to-blue-600 rounded-full blur-3xl opacity-20 animate-blob"></div>
                                <div class="absolute w-32 h-32 bg-gradient-to-r from-cyan-400 to-blue-400 rounded-full blur-3xl opacity-20 animate-blob animation-delay-2000"></div>
                                <div class="absolute w-32 h-32 bg-gradient-to-r from-blue-300 to-cyan-300 rounded-full blur-3xl opacity-20 animate-blob animation-delay-4000"></div>
                                
                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-24 h-24 border-2 border-blue-200 rounded-full" style="animation: spin-slow 20s linear infinite;"></div>
                                    <div class="absolute w-16 h-16 border-2 border-cyan-200 rounded-full" style="animation: spin-reverse 15s linear infinite;"></div>
                                    <div class="absolute w-8 h-8 bg-gradient-to-r from-blue-600 to-cyan-600 rounded-full"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Features Section
            <div id="features" class="py-24 bg-gray-50 border-t border-gray-200">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-20">
                        <h2 class="text-5xl font-bold text-gray-900 mb-6">
                            "Everything You Need"
                        </h2>
                        <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                            "Leptos provides all the tools you need to build fast, scalable web applications."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // Feature 1
                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-blue-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"⚙️"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Full-Stack Rust"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Write both frontend and backend in Rust. Get compile-time safety and type checking across your entire application."
                            </p>
                        </div>

                        // Feature 2
                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-cyan-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"⚡"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Lightning Fast"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Server-side rendering for instant page loads. Client-side hydration for seamless interactivity. Optimized for performance."
                            </p>
                        </div>

                        // Feature 3
                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-blue-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"🌍"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Global Edge"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Deploy on Cloudflare Workers for global distribution. Serve your app from 300+ data centers worldwide with instant propagation."
                            </p>
                        </div>

                        // Feature 4
                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-cyan-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"🔒"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Type Safe"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Leverage Rust's powerful type system to prevent entire categories of bugs. Compile-time verification across your stack."
                            </p>
                        </div>

                        // Feature 5
                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-blue-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"📦"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Reactive UI"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Build reactive components with automatic reactivity tracking. No manual state management boilerplate required."
                            </p>
                        </div>

                        // Feature 6
                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-cyan-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"🚀"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Zero Config"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Get started in minutes with sensible defaults. No complex webpack configs or build pipelines to manage."
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // How It Works
            <div id="how-it-works" class="py-24 bg-white">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-20">
                        <h2 class="text-5xl font-bold text-gray-900 mb-6">
                            "How It Works"
                        </h2>
                        <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                            "Simple, powerful, and elegant. From development to production."
                        </p>
                    </div>

                    <div class="space-y-12">
                        // Step 1
                        <div class="flex gap-8 items-start">
                            <div class="flex-shrink-0">
                                <div class="flex items-center justify-center h-16 w-16 rounded-full bg-blue-600 text-white font-bold text-xl">
                                    "1"
                                </div>
                            </div>
                            <div class="flex-1 pt-2">
                                <h3 class="text-2xl font-bold text-gray-900 mb-3">
                                    "Write Your App in Rust"
                                </h3>
                                <p class="text-gray-600 text-lg leading-relaxed">
                                    "Use Leptos to write your frontend and backend in Rust. Enjoy the full power of "
                                    "type safety, pattern matching, and zero-cost abstractions. Components are simple, "
                                    "reactive functions that automatically track dependencies."
                                </p>
                            </div>
                        </div>

                        // Step 2
                        <div class="flex gap-8 items-start">
                            <div class="flex-shrink-0">
                                <div class="flex items-center justify-center h-16 w-16 rounded-full bg-cyan-600 text-white font-bold text-xl">
                                    "2"
                                </div>
                            </div>
                            <div class="flex-1 pt-2">
                                <h3 class="text-2xl font-bold text-gray-900 mb-3">
                                    "Compile to WebAssembly"
                                </h3>
                                <p class="text-gray-600 text-lg leading-relaxed">
                                    "Leptos compiles your Rust code to ultra-efficient WebAssembly. Server-side rendering "
                                    "generates fast initial HTML responses. Client-side code hydrates in seconds with minimal JavaScript."
                                </p>
                            </div>
                        </div>

                        // Step 3
                        <div class="flex gap-8 items-start">
                            <div class="flex-shrink-0">
                                <div class="flex items-center justify-center h-16 w-16 rounded-full bg-blue-600 text-white font-bold text-xl">
                                    "3"
                                </div>
                            </div>
                            <div class="flex-1 pt-2">
                                <h3 class="text-2xl font-bold text-gray-900 mb-3">
                                    "Deploy to Cloudflare"
                                </h3>
                                <p class="text-gray-600 text-lg leading-relaxed">
                                    "With a single command, deploy your application to Cloudflare Workers. Your app runs on their global edge network, "
                                    "serving users from locations closest to them for minimal latency."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // CTA Section
            <div class="py-24 bg-blue-600">
                <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <h2 class="text-5xl font-bold text-white mb-6">
                        "Ready to Build?"
                    </h2>
                    <p class="text-2xl text-blue-100 mb-12 max-w-2xl mx-auto">
                        "Start building your next web application with Leptos and Cloudflare Workers today."
                    </p>
                    <a href="https://github.com/frontmesh/leptos-cloudflare-workers" target="_blank" rel="noopener noreferrer" class="inline-block px-10 py-4 bg-white text-blue-600 font-bold text-lg rounded-xl hover:bg-gray-50 transition shadow-lg hover:shadow-xl hover:shadow-white/30">
                        "Get Started Free"
                    </a>
                </div>
            </div>

            // Footer
            <footer class="bg-gray-900 text-gray-300 py-16 border-t border-gray-800">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-12 mb-12">
                        <div>
                            <div class="text-xl font-bold text-white mb-4">
                                "LeptosCloudflareWorkers"
                            </div>
                            <p class="text-gray-400">
                                "Full-stack web development in Rust."
                            </p>
                        </div>
                        <div>
                            <h4 class="text-white font-semibold mb-4">
                                "Product"
                            </h4>
                            <ul class="space-y-2">
                                <li>
                                    <a href="#" class="text-gray-400 hover:text-white transition">
                                        "Features"
                                    </a>
                                </li>
                                <li>
                                    <a href="#" class="text-gray-400 hover:text-white transition">
                                        "Security"
                                    </a>
                                </li>
                            </ul>
                        </div>
                        <div>
                            <h4 class="text-white font-semibold mb-4">
                                "Resources"
                            </h4>
                            <ul class="space-y-2">
                                <li>
                                    <a href="https://leptos.dev/01_getting_started.html" target="_blank" rel="noopener noreferrer" class="text-gray-400 hover:text-white transition">
                                        "Leptos Docs"
                                    </a>
                                </li>
                                <li>
                                    <a href="https://developers.cloudflare.com/workers/" target="_blank" rel="noopener noreferrer" class="text-gray-400 hover:text-white transition">
                                        "Workers Docs"
                                    </a>
                                </li>
                                <li>
                                    <a href="https://github.com/frontmesh/leptos-cloudflare-workers" target="_blank" rel="noopener noreferrer" class="text-gray-400 hover:text-white transition">
                                        "GitHub"
                                    </a>
                                </li>
                            </ul>
                        </div>
                        <div>
                            <h4 class="text-white font-semibold mb-4">
                                "Company"
                            </h4>
                            <ul class="space-y-2">
                                <li>
                                    <a href="#" class="text-gray-400 hover:text-white transition">
                                        "About"
                                    </a>
                                </li>
                                <li>
                                    <a href="#" class="text-gray-400 hover:text-white transition">
                                        "Blog"
                                    </a>
                                </li>
                                <li>
                                    <a href="#" class="text-gray-400 hover:text-white transition">
                                        "Contact"
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>

                    <div class="border-t border-gray-800 pt-8">
                        <div class="flex flex-col md:flex-row justify-between items-center">
                            <p class="text-gray-400 text-sm">
                                "© 2025 Leptos. All rights reserved."
                            </p>
                            <div class="flex space-x-6 mt-4 md:mt-0">
                                <a href="#" class="text-gray-400 hover:text-white transition text-sm">
                                    "Privacy"
                                </a>
                                <a href="#" class="text-gray-400 hover:text-white transition text-sm">
                                    "Terms"
                                </a>
                                <a href="#" class="text-gray-400 hover:text-white transition text-sm">
                                    "Status"
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    }
}
