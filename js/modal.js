/*** Auth Modal Window ***/

// NOTE: For the time being the modal system is only for user authentication.

var authModalWrapper = {
	element: null,

	visibleClass: "modal-visible",

	getElement() {
		this.element = this.element || document.getElementById("auth-modal-wrapper");
		return this.element;
	},

	isVisible() {
		return this.getElement().classList.contains(this.visibleClass);
	},
	show(sectionId) {
		this.getElement().classList.add(this.visibleClass);
		this.showSection(sectionId);
		this.disableScreenOverflow();
	},
	hide() {
		this.getElement().classList.remove(this.visibleClass);
		this.hideSections();
		this.enableScreenOverflow();
	},

	showSection(id) {
		document.getElementById(id).classList.add("auth-section-visible");
	},
	hideSections() {
		for (let section of document.querySelectorAll(".auth-section")) {
			section.classList.remove("auth-section-visible");

			let form = section.querySelector('.form');
			form.reset();
		}
	},
	changeSectionTo(id) {
		this.hideSections();
		this.showSection(id);
	},

	enableScreenOverflow() {
		document.body.style.overflow = "scroll";
	},
	disableScreenOverflow() {
		document.body.style.overflow = "hidden";
	}
}
