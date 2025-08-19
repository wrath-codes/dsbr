// Custom JavaScript for ArrowSUS Documentation

(function() {
    'use strict';

    // Add copy button to code blocks
    function addCopyButtons() {
        const codeBlocks = document.querySelectorAll('pre code');
        
        codeBlocks.forEach(function(codeBlock) {
            const pre = codeBlock.parentNode;
            const button = document.createElement('button');
            
            button.className = 'copy-button';
            button.textContent = 'Copy';
            button.setAttribute('aria-label', 'Copy code to clipboard');
            
            button.addEventListener('click', function() {
                const text = codeBlock.textContent;
                
                if (navigator.clipboard) {
                    navigator.clipboard.writeText(text).then(function() {
                        button.textContent = 'Copied!';
                        setTimeout(function() {
                            button.textContent = 'Copy';
                        }, 2000);
                    });
                } else {
                    // Fallback for older browsers
                    const textArea = document.createElement('textarea');
                    textArea.value = text;
                    document.body.appendChild(textArea);
                    textArea.select();
                    document.execCommand('copy');
                    document.body.removeChild(textArea);
                    
                    button.textContent = 'Copied!';
                    setTimeout(function() {
                        button.textContent = 'Copy';
                    }, 2000);
                }
            });
            
            pre.style.position = 'relative';
            pre.appendChild(button);
        });
    }

    // Add external link indicators
    function addExternalLinkIndicators() {
        const links = document.querySelectorAll('a[href^="http"]');
        
        links.forEach(function(link) {
            if (!link.hostname.includes(window.location.hostname)) {
                link.setAttribute('target', '_blank');
                link.setAttribute('rel', 'noopener noreferrer');
                
                const icon = document.createElement('span');
                icon.innerHTML = ' ↗';
                icon.style.fontSize = '0.8em';
                icon.style.opacity = '0.7';
                link.appendChild(icon);
            }
        });
    }

    // Smooth scrolling for anchor links
    function enableSmoothScrolling() {
        const anchorLinks = document.querySelectorAll('a[href^="#"]');
        
        anchorLinks.forEach(function(link) {
            link.addEventListener('click', function(e) {
                const targetId = this.getAttribute('href').substring(1);
                const targetElement = document.getElementById(targetId);
                
                if (targetElement) {
                    e.preventDefault();
                    targetElement.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                    
                    // Update URL without jumping
                    history.pushState(null, null, '#' + targetId);
                }
            });
        });
    }

    // Initialize when DOM is ready
    function init() {
        addCopyButtons();
        addExternalLinkIndicators();
        enableSmoothScrolling();
    }

    // Run initialization
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }

    // Re-run on page navigation (for SPA-like behavior)
    window.addEventListener('popstate', function() {
        setTimeout(init, 100);
    });

})();

// Add CSS for copy buttons
const style = document.createElement('style');
style.textContent = `
    .copy-button {
        position: absolute;
        top: 0.5rem;
        right: 0.5rem;
        background: rgba(0, 0, 0, 0.7);
        color: white;
        border: none;
        padding: 0.25rem 0.5rem;
        border-radius: 3px;
        font-size: 0.75rem;
        cursor: pointer;
        opacity: 0;
        transition: opacity 0.2s;
    }
    
    pre:hover .copy-button {
        opacity: 1;
    }
    
    .copy-button:hover {
        background: rgba(0, 0, 0, 0.9);
    }
    
    .copy-button:active {
        transform: scale(0.95);
    }
`;
document.head.appendChild(style);