// R Commerce Documentation - Extra JavaScript

// Add copy button to code blocks
document.addEventListener('DOMContentLoaded', function() {
  // Highlight current navigation item
  const currentPath = window.location.pathname;
  const navLinks = document.querySelectorAll('.md-nav__link');
  
  navLinks.forEach(link => {
    if (link.getAttribute('href') === currentPath) {
      link.classList.add('md-nav__link--active');
    }
  });
  
  // Add version selector handler
  const versionSelector = document.querySelector('.md-version__current');
  if (versionSelector) {
    versionSelector.addEventListener('change', function(e) {
      const selectedVersion = e.target.value;
      // Version switching logic would go here
      console.log('Switching to version:', selectedVersion);
    });
  }
});

// Mermaid diagram configuration
window.mermaidConfig = {
  startOnLoad: true,
  theme: 'default',
  themeVariables: {
    primaryColor: '#6366f1',
    primaryTextColor: '#fff',
    primaryBorderColor: '#4f46e5',
    lineColor: '#6b7280',
    secondaryColor: '#8b5cf6',
    tertiaryColor: '#ec4899'
  },
  flowchart: {
    useMaxWidth: true,
    htmlLabels: true,
    curve: 'basis'
  },
  sequence: {
    useMaxWidth: true,
    diagramMarginX: 50,
    diagramMarginY: 10,
    actorMargin: 50,
    width: 150,
    height: 65,
    boxMargin: 10,
    boxTextMargin: 5,
    noteMargin: 10,
    messageMargin: 35,
    mirrorActors: true,
    bottomMarginAdj: 1,
    useMaxWidth: true,
    rightAngles: false,
    showSequenceNumbers: false
  }
};

// Search enhancement
document.addEventListener('keydown', function(e) {
  // Ctrl/Cmd + K to open search
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault();
    const searchInput = document.querySelector('.md-search__input');
    if (searchInput) {
      searchInput.focus();
    }
  }
});

// Print-friendly handling
window.addEventListener('beforeprint', function() {
  document.body.classList.add('printing');
});

window.addEventListener('afterprint', function() {
  document.body.classList.remove('printing');
});

// Table of contents scroll spy
function initTocSpy() {
  const headings = document.querySelectorAll('.md-typeset h2[id], .md-typeset h3[id]');
  const tocLinks = document.querySelectorAll('.md-nav--secondary .md-nav__link');
  
  if (!headings.length || !tocLinks.length) return;
  
  let currentId = '';
  
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        currentId = entry.target.id;
        updateToc(currentId);
      }
    });
  }, {
    rootMargin: '-20% 0% -80% 0%'
  });
  
  headings.forEach(heading => observer.observe(heading));
  
  function updateToc(id) {
    tocLinks.forEach(link => {
      link.classList.remove('md-nav__link--active');
      if (link.getAttribute('href') === '#' + id) {
        link.classList.add('md-nav__link--active');
      }
    });
  }
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initTocSpy);
} else {
  initTocSpy();
}
