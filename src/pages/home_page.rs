use leptos::prelude::*;
use leptos::{component, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
            // Navigation
            <nav class="bg-slate-950 bg-opacity-50 backdrop-blur-md border-b border-slate-700">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
                    <div class="flex justify-between items-center">
                        <div class="text-2xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-cyan-400">
                            "Leptos CF"
                        </div>
                        <div class="hidden md:flex space-x-8">
                            <a href="#features" class="text-slate-300 hover:text-white transition">
                                "Features"
                            </a>
                            <a href="#tech" class="text-slate-300 hover:text-white transition">
                                "Technology"
                            </a>
                            <a href="#about" class="text-slate-300 hover:text-white transition">
                                "About"
                            </a>
                        </div>
                    </div>
                </div>
            </nav>

            // Hero Section
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-12 items-center">
                    <div class="space-y-6">
                        <h1 class="text-5xl md:text-6xl font-bold text-white leading-tight">
                            "Full-Stack Web with "
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 via-purple-400 to-pink-400">
                                "Leptos"
                            </span>
                        </h1>
                        <p class="text-xl text-slate-300 leading-relaxed">
                            "Experience the power of server-side rendering and client-side hydration "
                            "on Cloudflare Workers. Building fast, modern web applications has never been easier."
                        </p>
                        <div class="flex flex-wrap gap-4 pt-4">
                            <button class="px-8 py-3 bg-gradient-to-r from-blue-500 to-blue-600 text-white font-semibold rounded-lg hover:from-blue-600 hover:to-blue-700 transition shadow-lg hover:shadow-blue-500/50">
                                "Get Started"
                            </button>
                            <button class="px-8 py-3 bg-slate-700 text-white font-semibold rounded-lg hover:bg-slate-600 transition border border-slate-600">
                                "Learn More"
                            </button>
                        </div>
                    </div>
                    <div class="relative">
                        <div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-cyan-500 rounded-2xl blur-3xl opacity-20"></div>
                        <div class="relative bg-slate-800 border border-slate-700 rounded-2xl p-8 space-y-4">
                            <div class="h-2 bg-slate-700 rounded w-3/4"></div>
                            <div class="h-2 bg-slate-700 rounded w-full"></div>
                            <div class="h-2 bg-slate-700 rounded w-5/6"></div>
                            <div class="pt-4 space-y-3">
                                <div class="h-12 bg-gradient-to-r from-blue-500/20 to-cyan-500/20 rounded"></div>
                                <div class="h-12 bg-gradient-to-r from-purple-500/20 to-pink-500/20 rounded"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Features Section
            <div id="features" class="bg-slate-800 bg-opacity-50 py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-white text-center mb-16">
                        "Why Choose Leptos?"
                    </h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        // Feature 1
                        <div class="group">
                            <div class="bg-slate-700 bg-opacity-50 p-8 rounded-xl border border-slate-600 hover:border-blue-400 transition group-hover:bg-slate-600">
                                <div class="w-12 h-12 bg-gradient-to-br from-blue-400 to-cyan-400 rounded-lg flex items-center justify-center mb-4">
                                    <span class="text-white font-bold text-xl">"⚡"</span>
                                </div>
                                <h3 class="text-xl font-bold text-white mb-3">
                                    "Lightning Fast"
                                </h3>
                                <p class="text-slate-300">
                                    "Optimized for performance with server-side rendering and minimal JavaScript."
                                </p>
                            </div>
                        </div>

                        // Feature 2
                        <div class="group">
                            <div class="bg-slate-700 bg-opacity-50 p-8 rounded-xl border border-slate-600 hover:border-purple-400 transition group-hover:bg-slate-600">
                                <div class="w-12 h-12 bg-gradient-to-br from-purple-400 to-pink-400 rounded-lg flex items-center justify-center mb-4">
                                    <span class="text-white font-bold text-xl">"🔧"</span>
                                </div>
                                <h3 class="text-xl font-bold text-white mb-3">
                                    "Full-Stack Rust"
                                </h3>
                                <p class="text-slate-300">
                                    "Write both frontend and backend in Rust for maximum type safety and performance."
                                </p>
                            </div>
                        </div>

                        // Feature 3
                        <div class="group">
                            <div class="bg-slate-700 bg-opacity-50 p-8 rounded-xl border border-slate-600 hover:border-green-400 transition group-hover:bg-slate-600">
                                <div class="w-12 h-12 bg-gradient-to-br from-green-400 to-emerald-400 rounded-lg flex items-center justify-center mb-4">
                                    <span class="text-white font-bold text-xl">"☁️"</span>
                                </div>
                                <h3 class="text-xl font-bold text-white mb-3">
                                    "Edge Computing"
                                </h3>
                                <p class="text-slate-300">
                                    "Deploy instantly to Cloudflare Workers at the edge for global availability."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Tech Stack Section
            <div id="tech" class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <h2 class="text-4xl font-bold text-white text-center mb-16">
                    "Modern Tech Stack"
                </h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                    <div class="flex items-center justify-center p-6 bg-slate-800 bg-opacity-50 rounded-lg border border-slate-700 hover:border-slate-500 transition">
                        <span class="text-white font-semibold">"Leptos"</span>
                    </div>
                    <div class="flex items-center justify-center p-6 bg-slate-800 bg-opacity-50 rounded-lg border border-slate-700 hover:border-slate-500 transition">
                        <span class="text-white font-semibold">"Rust"</span>
                    </div>
                    <div class="flex items-center justify-center p-6 bg-slate-800 bg-opacity-50 rounded-lg border border-slate-700 hover:border-slate-500 transition">
                        <span class="text-white font-semibold">"Tailwind"</span>
                    </div>
                    <div class="flex items-center justify-center p-6 bg-slate-800 bg-opacity-50 rounded-lg border border-slate-700 hover:border-slate-500 transition">
                        <span class="text-white font-semibold">"Cloudflare"</span>
                    </div>
                </div>
            </div>

            // CTA Section
            <div class="bg-gradient-to-r from-blue-600 to-purple-600 py-16">
                <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <h2 class="text-4xl font-bold text-white mb-6">
                        "Ready to Build Something Great?"
                    </h2>
                    <p class="text-xl text-blue-100 mb-8">
                        "Start building with Leptos and Cloudflare Workers today."
                    </p>
                    <button class="px-10 py-4 bg-white text-blue-600 font-bold rounded-lg hover:bg-slate-100 transition shadow-lg">
                        "Start Now"
                    </button>
                </div>
            </div>

            // Footer
            <footer class="bg-slate-950 border-t border-slate-800 py-8">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex flex-col md:flex-row justify-between items-center">
                        <div class="text-slate-400 mb-4 md:mb-0">
                            "© 2025 Leptos Cloudflare Workers. All rights reserved."
                        </div>
                        <div class="flex space-x-6">
                            <a href="#" class="text-slate-400 hover:text-white transition">
                                "Twitter"
                            </a>
                            <a href="#" class="text-slate-400 hover:text-white transition">
                                "GitHub"
                            </a>
                            <a href="#" class="text-slate-400 hover:text-white transition">
                                "Docs"
                            </a>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    }
}
