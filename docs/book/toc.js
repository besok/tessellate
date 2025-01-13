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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="start.html"><strong aria-hidden="true">2.</strong> Definitions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="mesh.html"><strong aria-hidden="true">2.1.</strong> Mesh</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="parts.html"><strong aria-hidden="true">2.1.1.</strong> Parts</a></li><li class="chapter-item expanded "><a href="properties.html"><strong aria-hidden="true">2.1.2.</strong> Properties</a></li><li class="chapter-item expanded "><a href="attributes.html"><strong aria-hidden="true">2.1.3.</strong> Attributes</a></li><li class="chapter-item expanded "><a href="query.html"><strong aria-hidden="true">2.1.4.</strong> Query</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.5.</strong> Statistics</div></li></ol></li></ol></li><li class="chapter-item expanded "><a href="import.html"><strong aria-hidden="true">3.</strong> Import</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="stl.html"><strong aria-hidden="true">3.1.</strong> STL</a></li><li class="chapter-item expanded "><a href="ply.html"><strong aria-hidden="true">3.2.</strong> PLY</a></li><li class="chapter-item expanded "><a href="obj.html"><strong aria-hidden="true">3.3.</strong> OBJ</a></li></ol></li><li class="chapter-item expanded "><a href="visualization.html"><strong aria-hidden="true">4.</strong> Visualization</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="options.html"><strong aria-hidden="true">4.1.</strong> Options</a></li><li class="chapter-item expanded "><a href="controls.html"><strong aria-hidden="true">4.2.</strong> Controls</a></li></ol></li><li class="chapter-item expanded "><a href="mesh_manipul.html"><strong aria-hidden="true">5.</strong> Mesh manipulation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="basic_geometries.html"><strong aria-hidden="true">5.1.</strong> Basic Geometries</a></li><li class="chapter-item expanded "><a href="parametric_geometric_objects.html"><strong aria-hidden="true">5.2.</strong> Parametric Geometric Objects</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.3.</strong> Explicit Structured Grid</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.4.</strong> Structured Surface</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.5.</strong> Triangulated Surface</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.6.</strong> Platonic Solids</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.7.</strong> Point Cloud</div></li></ol></li><li class="chapter-item expanded "><a href="mesh_filtering.html"><strong aria-hidden="true">6.</strong> Mesh filtering</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="boolean_operations.html"><strong aria-hidden="true">6.1.</strong> Boolean Operations</a></li><li class="chapter-item expanded "><a href="extract_cell_centers.html"><strong aria-hidden="true">6.2.</strong> Extract Polygon and Edge Centers</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.3.</strong> Clipping with a Surface, plane and boxes</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.4.</strong> Collision Detection</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.5.</strong> Volumetric Analysis</div></li><li class="chapter-item expanded "><a href="connectivity.html"><strong aria-hidden="true">6.6.</strong> Connectivity</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.7.</strong> Decimate a mesh</div></li><li class="chapter-item expanded "><a href="extract_edges.html"><strong aria-hidden="true">6.8.</strong> Extract Edges</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.9.</strong> Extract Surface</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.10.</strong> Gaussian Smoothing</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.11.</strong> Geodesic Paths</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.12.</strong> Interpolating</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.13.</strong> Computing Mesh Quality</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.14.</strong> Resampling</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.15.</strong> Surface Smoothing</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.16.</strong> Surface Reconstruction</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.17.</strong> Voxelize a Surface Mesh</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.18.</strong> Subdivide Cells</div></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">7.</strong> Geometric quantities</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">7.1.</strong> Laplacian smoothing</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">7.2.</strong> Gaussian Curvature</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">7.3.</strong> Gradient</div></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">8.</strong> Ray tracing</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">8.1.</strong> Ray Casting</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">8.2.</strong> Ray Tracing</div></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">9.</strong> Visualize the Moeller-Trumbore Algorithm</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">10.</strong> Tesselate</div></li><li class="chapter-item expanded "><a href="auxiliary.html"><strong aria-hidden="true">11.</strong> Auxiliary Tools and Structures</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="kdtree.html"><strong aria-hidden="true">11.1.</strong> KDTree</a></li><li class="chapter-item expanded "><a href="bsptree.html"><strong aria-hidden="true">11.2.</strong> BSP Tree</a></li><li class="chapter-item expanded "><a href="distance.html"><strong aria-hidden="true">11.3.</strong> Distance</a></li></ol></li><li class="chapter-item expanded "><a href="CONTRIBUTING.html"><strong aria-hidden="true">12.</strong> Contributing</a></li><li class="chapter-item expanded "><a href="LICENSE.html"><strong aria-hidden="true">13.</strong> License</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
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
