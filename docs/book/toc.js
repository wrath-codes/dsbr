// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="getting-started/installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="getting-started/quick-start.html"><strong aria-hidden="true">2.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="getting-started/configuration.html"><strong aria-hidden="true">3.</strong> Configuration</a></li><li class="chapter-item expanded affix "><li class="part-title">Core Concepts</li><li class="chapter-item expanded "><a href="core/architecture.html"><strong aria-hidden="true">4.</strong> Architecture Overview</a></li><li class="chapter-item expanded "><a href="core/domain-models.html"><strong aria-hidden="true">5.</strong> Domain Models</a></li><li class="chapter-item expanded "><a href="core/error-handling.html"><strong aria-hidden="true">6.</strong> Error Handling</a></li><li class="chapter-item expanded "><a href="core/result-types.html"><strong aria-hidden="true">7.</strong> Result Types</a></li><li class="chapter-item expanded affix "><li class="part-title">Utilities</li><li class="chapter-item expanded "><a href="utils/time.html"><strong aria-hidden="true">8.</strong> Time Utilities</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="utils/time/month.html"><strong aria-hidden="true">8.1.</strong> Month Operations</a></li></ol></li><li class="chapter-item expanded "><a href="utils/path.html"><strong aria-hidden="true">9.</strong> Path Utilities</a></li><li class="chapter-item expanded "><a href="utils/meta.html"><strong aria-hidden="true">10.</strong> Meta Utilities</a></li><li class="chapter-item expanded affix "><li class="part-title">Data Processing</li><li class="chapter-item expanded "><a href="data/file-handling.html"><strong aria-hidden="true">11.</strong> File Handling</a></li><li class="chapter-item expanded "><a href="data/parsing.html"><strong aria-hidden="true">12.</strong> Parsing</a></li><li class="chapter-item expanded "><a href="data/schemas.html"><strong aria-hidden="true">13.</strong> Schemas</a></li><li class="chapter-item expanded affix "><li class="part-title">IO Operations</li><li class="chapter-item expanded "><a href="io/ftp.html"><strong aria-hidden="true">14.</strong> FTP Integration</a></li><li class="chapter-item expanded "><a href="io/s3.html"><strong aria-hidden="true">15.</strong> S3 Integration</a></li><li class="chapter-item expanded "><a href="io/filesystem.html"><strong aria-hidden="true">16.</strong> Local File System</a></li><li class="chapter-item expanded affix "><li class="part-title">Advanced Topics</li><li class="chapter-item expanded "><a href="advanced/polars.html"><strong aria-hidden="true">17.</strong> Polars Integration</a></li><li class="chapter-item expanded "><a href="advanced/arrow.html"><strong aria-hidden="true">18.</strong> Arrow Integration</a></li><li class="chapter-item expanded "><a href="advanced/performance.html"><strong aria-hidden="true">19.</strong> Performance Optimization</a></li><li class="chapter-item expanded "><a href="advanced/testing.html"><strong aria-hidden="true">20.</strong> Testing Strategies</a></li><li class="chapter-item expanded affix "><li class="part-title">API Reference</li><li class="chapter-item expanded "><a href="api/core.html"><strong aria-hidden="true">21.</strong> Core API</a></li><li class="chapter-item expanded "><a href="api/utils.html"><strong aria-hidden="true">22.</strong> Utils API</a></li><li class="chapter-item expanded "><a href="api/domain.html"><strong aria-hidden="true">23.</strong> Domain API</a></li><li class="chapter-item expanded affix "><li class="part-title">Examples</li><li class="chapter-item expanded "><a href="examples/basic.html"><strong aria-hidden="true">24.</strong> Basic Usage</a></li><li class="chapter-item expanded "><a href="examples/pipeline.html"><strong aria-hidden="true">25.</strong> Data Processing Pipeline</a></li><li class="chapter-item expanded "><a href="examples/ftp-processing.html"><strong aria-hidden="true">26.</strong> FTP File Processing</a></li><li class="chapter-item expanded "><a href="examples/custom-parsers.html"><strong aria-hidden="true">27.</strong> Custom Parsers</a></li><li class="chapter-item expanded affix "><li class="part-title">Contributing</li><li class="chapter-item expanded "><a href="contributing/setup.html"><strong aria-hidden="true">28.</strong> Development Setup</a></li><li class="chapter-item expanded "><a href="contributing/style.html"><strong aria-hidden="true">29.</strong> Code Style</a></li><li class="chapter-item expanded "><a href="contributing/testing.html"><strong aria-hidden="true">30.</strong> Testing</a></li><li class="chapter-item expanded "><a href="contributing/documentation.html"><strong aria-hidden="true">31.</strong> Documentation</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><a href="changelog.html">Changelog</a></li><li class="chapter-item expanded affix "><a href="license.html">License</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
