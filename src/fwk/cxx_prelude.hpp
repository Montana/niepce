/*
 * niepce - fwk/cxx_prelude.hpp
 */

#pragma once

// things that need to be declared before anything.
// early "extern C++"
// And that the implementation needs too.
namespace fwk {
class Moniker;
class PropertyValue;
class WrappedPropertyBag;
}

namespace eng {
class Label;
class LcChannel;
class LibNotification;
class ThumbnailCache;
}

namespace npc {
class LnListener;
}
