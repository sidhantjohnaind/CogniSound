/*! QRious v4.0.2 | (C) 2017 Alasdair Mercer | GPL v3 License */
!function(t,e){"object"==typeof exports&&"undefined"!=typeof module?module.exports=e():"function"==typeof define&&define.amd?define(e):t.QRious=e()}(this,function(){"use strict";function t(t,e){var n;return"function"==typeof Object.create?n=Object.create(t):(s.prototype=t,n=new s,s.prototype=null),e&&i(!0,n,e),n}function e(e,n,s,r){var o=this;return"string"!=typeof e&&(r=s,s=n,n=e,e=null),"function"!=typeof n&&(r=s,s=n,n=function(){return o.apply(this,arguments)}),i(!1,n,o,r),n.prototype=t(o.prototype,s),n.prototype.constructor=n,n.class_=e||o.class_,n.super_=o,n}function i(t,e,i){for(var n,s,a=0,h=(i=o.call(arguments,2)).length;a<h;a++){s=i[a];for(n in s)t&&!r.call(s,n)||(e[n]=s[n])}}function n(){}var s=function(){},r=Object.prototype.hasOwnProperty,o=Array.prototype.slice,a=e;n.class_="Nevis",n.super_=Object,n.extend=a;var h=n,f=h.extend(function(t,e,i){this.qrious=t,this.element=e,this.element.qrious=t,this.enabled=Boolean(i)},{draw:function(t){},getElement:function(){return this.enabled||(this.enabled=!0,this.render()),this.element},getModuleSize:function(t){var e=this.qrious,i=e.padding||0,n=Math.floor((e.size-2*i)/t.width);return Math.max(1,n)},getOffset:function(t){var e=this.qrious,i=e.padding;if(null!=i)return i;var n=this.getModuleSize(t),s=Math.floor((e.size-n*t.width)/2);return Math.max(0,s)},render:function(t){this.enabled&&(this.resize(),this.reset(),this.draw(t))},reset:function(){},resize:function(){}}),c=f.extend({draw:function(t){var e,i,n=this.qrious,s=this.getModuleSize(t),r=this.getOffset(t),o=this.element.getContext("2d");for(o.fillStyle=n.foreground,o.globalAlpha=n.foregroundAlpha,e=0;e<t.width;e++)for(i=0;i<t.width;i++)t.buffer[i*t.width+e]&&o.fillRect(s*e+r,s*i+r,s,s)},reset:function(){var t=this.qrious,e=this.element.getContext("2d"),i=t.size;e.lineWidth=1,e.clearRect(0,0,i,i),e.fillStyle=t.background,e.globalAlpha=t.backgroundAlpha,e.fillRect(0,0,i,i)},resize:function(){var t=this.element;t.width=t.height=this.qrious.size}}),u=h.extend(null,{BLOCK:[0,11,15,19,23,27,31,16,18,20,22,24,26,28,20,22,24,24,26,28,28,22,24,24,26,26,28,28,24,24,26,26,26,28,28,24,26,26,26,28,28]}),l=h.extend(null,{BLOCKS:[1,0,19,7,1,0,16,10,1,0,13,13,1,0,9,17,1,0,34,10,1,0,28,16,1,0,22,22,1,0,16,28,1,0,55,15,1,0,44,26,2,0,17,18,2,0,13,22,1,0,80,20,2,0,32,18,2,0,24,26,4,0,9,16,1,0,108,26,2,0,43,24,2,2,15,18,2,2,11,22,2,0,68,18,4,0,27,16,4,0,19,24,4,0,15,28,2,0,78,20,4,0,31,18,2,4,14,18,4,1,13,26,2,0,97,24,2,2,38,22,4,2,18,22,4,2,14,26,2,0,116,30,3,2,36,22,4,4,16,20,4,4,12,24,2,2,68,18,4,1,43,26,6,2,19,24,6,2,15,28,4,0,81,20,1,4,50,30,4,4,22,28,3,8,12,24,2,2,92,24,6,2,36,22,4,6,20,26,7,4,14,28,4,0,107,26,8,1,37,22,8,4,20,24,12,4,11,22,3,1,115,30,4,5,40,24,11,5,16,20,11,5,12,24,5,1,87,22,5,5,41,24,5,7,24,30,11,7,12,24,5,1,98,24,7,3,45,28,15,2,19,24,3,13,15,30,1,5,107,28,10,1,46,28,1,15,22,28,2,17,14,28,5,1,120,30,9,4,43,26,17,1,22,28,2,19,14,28,3,4,113,28,3,11,44,26,17,4,21,26,9,16,13,26,3,5,107,28,3,13,41,26,15,5,24,30,15,10,15,28,4,4,116,28,17,0,42,26,17,6,22,28,19,6,16,30,2,7,111,28,17,0,46,28,7,16,24,30,34,0,13,24,4,5,121,30,4,14,47,28,11,14,24,30,16,14,15,30,6,4,117,30,6,14,45,28,11,16,24,30,30,2,16,30,8,4,106,26,8,13,47,28,7,22,24,30,22,13,15,30,10,2,114,28,19,4,46,28,28,6,22,28,33,4,16,30,8,4,122,30,22,3,45,28,8,26,23,30,12,28,15,30,3,10,117,30,3,23,45,28,4,31,24,30,11,31,15,30,7,7,116,30,21,7,45,28,1,37,23,30,19,26,15,30,5,10,115,30,19,10,47,28,15,25,24,30,23,25,15,30,13,3,115,30,2,29,46,28,42,1,24,30,23,28,15,30,17,0,115,30,10,23,46,28,10,35,24,30,19,35,15,30,17,1,115,30,14,21,46,28,29,19,24,30,11,46,15,30,13,6,115,30,14,23,46,28,44,7,24,30,59,1,16,30,12,7,121,30,12,26,47,28,39,14,24,30,22,41,15,30,6,14,121,30,6,34,47,28,46,10,24,30,2,64,15,30,17,4,122,30,29,14,46,28,49,10,24,30,24,46,15,30,4,18,122,30,13,32,46,28,48,14,24,30,42,32,15,30,20,4,117,30,40,7,47,28,43,22,24,30,10,67,15,30,19,6,118,30,18,31,47,28,34,34,24,30,20,61,15,30],FINAL_FORMAT:[30660,29427,32170,30877,26159,25368,27713,26998,21522,20773,24188,23371,17913,16590,20375,19104,13663,12392,16177,14854,9396,8579,11994,11245,5769,5054,7399,6608,1890,597,3340,2107],LEVELS:{L:1,M:2,Q:3,H:4}}),_=h.extend(null,{EXPONENT:[1,2,4,8,16,32,64,128,29,58,116,232,205,135,19,38,76,152,45,90,180,117,234,201,143,3,6,12,24,48,96,192,157,39,78,156,37,74,148,53,106,212,181,119,238,193,159,35,70,140,5,10,20,40,80,160,93,186,105,210,185,111,222,161,95,190,97,194,153,47,94,188,101,202,137,15,30,60,120,240,253,231,211,187,107,214,177,127,254,225,223,163,91,182,113,226,217,175,67,134,17,34,68,136,13,26,52,104,208,189,103,206,129,31,62,124,248,237,199,147,59,118,236,197,151,51,102,204,133,23,46,92,184,109,218,169,79,158,33,66,132,21,42,84,168,77,154,41,82,164,85,170,73,146,57,114,228,213,183,115,230,209,191,99,198,145,63,126,252,229,215,179,123,246,241,255,227,219,171,75,150,49,98,196,149,55,110,220,165,87,174,65,130,25,50,100,200,141,7,14,28,56,112,224,221,167,83,166,81,162,89,178,121,242,249,239,195,155,43,86,172,69,138,9,18,36,72,144,61,122,244,245,247,243,251,235,203,139,11,22,44,88,176,125,250,233,207,131,27,54,108,216,173,71,142,0],LOG:[255,0,1,25,2,50,26,198,3,223,51,238,27,104,199,75,4,100,224,14,52,141,239,129,28,193,105,248,200,8,76,113,5,138,101,47,225,36,15,33,53,147,142,218,240,18,130,69,29,181,194,125,106,39,249,185,201,154,9,120,77,228,114,166,6,191,139,98,102,221,48,253,226,152,37,179,16,145,34,136,54,208,148,206,143,150,219,189,241,210,19,92,131,56,70,64,30,66,182,163,195,72,126,110,107,58,40,84,250,133,186,61,202,94,155,159,10,21,121,43,78,212,229,172,115,243,167,87,7,112,192,247,140,128,99,13,103,74,222,237,49,197,254,24,227,165,153,119,38,184,180,124,17,68,146,217,35,32,137,46,55,63,209,91,149,188,207,205,144,135,151,178,220,252,190,97,242,86,211,171,20,42,93,158,132,60,57,83,71,109,65,162,31,45,67,216,183,123,164,118,196,23,73,236,127,12,111,246,108,161,59,82,41,157,85,170,251,96,134,177,187,204,62,90,203,89,95,176,156,169,160,81,11,245,22,235,122,117,44,215,79,174,213,233,230,231,173,232,116,214,244,234,168,80,88,175]}),d=h.extend(null,{BLOCK:[3220,1468,2713,1235,3062,1890,2119,1549,2344,2936,1117,2583,1330,2470,1667,2249,2028,3780,481,4011,142,3098,831,3445,592,2517,1776,2234,1951,2827,1070,2660,1345,3177]}),v=h.extend(function(t){var e,i,n,s,r,o=t.value.length;for(this._badness=[],this._level=l.LEVELS[t.level],this._polynomial=[],this._value=t.value,this._version=0,this._stringBuffer=[];this._version<40&&(this._version++,n=4*(this._level-1)+16*(this._version-1),s=l.BLOCKS[n++],r=l.BLOCKS[n++],e=l.BLOCKS[n++],i=l.BLOCKS[n],n=e*(s+r)+r-3+(this._version<=9),!(o<=n)););this._dataBlock=e,this._eccBlock=i,this._neccBlock1=s,this._neccBlock2=r;var a=this.width=17+4*this._version;this.buffer=v._createArray(a*a),this._ecc=v._createArray(e+(e+i)*(s+r)+r),this._mask=v._createArray((a*(a+1)+1)/2),this._insertFinders(),this._insertAlignments(),this.buffer[8+a*(a-8)]=1,this._insertTimingGap(),this._reverseMask(),this._insertTimingRowAndColumn(),this._insertVersion(),this._syncMask(),this._convertBitStream(o),this._calculatePolynomial(),this._appendEccToData(),this._interleaveBlocks(),this._pack(),this._finish()},{_addAlignment:function(t,e){var i,n=this.buffer,s=this.width;for(n[t+s*e]=1,i=-2;i<2;i++)n[t+i+s*(e-2)]=1,n[t-2+s*(e+i+1)]=1,n[t+2+s*(e+i)]=1,n[t+i+1+s*(e+2)]=1;for(i=0;i<2;i++)this._setMask(t-1,e+i),this._setMask(t+1,e-i),this._setMask(t-i,e-1),this._setMask(t+i,e+1)},_appendData:function(t,e,i,n){var s,r,o,a=this._polynomial,h=this._stringBuffer;for(r=0;r<n;r++)h[i+r]=0;for(r=0;r<e;r++){if(255!==(s=_.LOG[h[t+r]^h[i]]))for(o=1;o<n;o++)h[i+o-1]=h[i+o]^_.EXPONENT[v._modN(s+a[n-o])];else for(o=i;o<i+n;o++)h[o]=h[o+1];h[i+n-1]=255===s?0:_.EXPONENT[v._modN(s+a[0])]}},_appendEccToData:function(){var t,e=0,i=this._dataBlock,n=this._calculateMaxLength(),s=this._eccBlock;for(t=0;t<this._neccBlock1;t++)this._appendData(e,i,n,s),e+=i,n+=s;for(t=0;t<this._neccBlock2;t++)this._appendData(e,i+1,n,s),e+=i+1,n+=s},_applyMask:function(t){var e,i,n,s,r=this.buffer,o=this.width;switch(t){case 0:for(s=0;s<o;s++)for(n=0;n<o;n++)n+s&1||this._isMasked(n,s)||(r[n+s*o]^=1);break;case 1:for(s=0;s<o;s++)for(n=0;n<o;n++)1&s||this._isMasked(n,s)||(r[n+s*o]^=1);break;case 2:for(s=0;s<o;s++)for(e=0,n=0;n<o;n++,e++)3===e&&(e=0),e||this._isMasked(n,s)||(r[n+s*o]^=1);break;case 3:for(i=0,s=0;s<o;s++,i++)for(3===i&&(i=0),e=i,n=0;n<o;n++,e++)3===e&&(e=0),e||this._isMasked(n,s)||(r[n+s*o]^=1);break;case 4:for(s=0;s<o;s++)for(e=0,i=s>>1&1,n=0;n<o;n++,e++)3===e&&(e=0,i=!i),i||this._isMasked(n,s)||(r[n+s*o]^=1);break;case 5:for(i=0,s=0;s<o;s++,i++)for(3===i&&(i=0),e=0,n=0;n<o;n++,e++)3===e&&(e=0),(n&s&1)+!(!e|!i)||this._isMasked(n,s)||(r[n+s*o]^=1);break;case 6:for(i=0,s=0;s<o;s++,i++)for(3===i&&(i=0),e=0,n=0;n<o;n++,e++)3===e&&(e=0),(n&s&1)+(e&&e===i)&1||this._isMasked(n,s)||(r[n+s*o]^=1);break;case 7:for(i=0,s=0;s<o;s++,i++)for(3===i&&(i=0),e=0,n=0;n<o;n++,e++)3===e&&(e=0),(e&&e===i)+(n+s&1)&1||this._isMasked(n,s)||(r[n+s*o]^=1)}},_calculateMaxLength:function(){return this._dataBlock*(this._neccBlock1+this._neccBlock2)+this._neccBlock2},_calculatePolynomial:function(){var t,e,i=this._eccBlock,n=this._polynomial;for(n[0]=1,t=0;t<i;t++){for(n[t+1]=1,e=t;e>0;e--)n[e]=n[e]?n[e-1]^_.EXPONENT[v._modN(_.LOG[n[e]]+t)]:n[e-1];n[0]=_.EXPONENT[v._modN(_.LOG[n[0]]+t)]}for(t=0;t<=i;t++)n[t]=_.LOG[n[t]]},_checkBadness:function(){var t,e,i,n,s,r=0,o=this._badness,a=this.buffer,h=this.width;for(s=0;s<h-1;s++)for(n=0;n<h-1;n++)(a[n+h*s]&&a[n+1+h*s]&&a[n+h*(s+1)]&&a[n+1+h*(s+1)]||!(a[n+h*s]||a[n+1+h*s]||a[n+h*(s+1)]||a[n+1+h*(s+1)]))&&(r+=v.N2);var f=0;for(s=0;s<h;s++){for(i=0,o[0]=0,t=0,n=0;n<h;n++)t===(e=a[n+h*s])?o[i]++:o[++i]=1,f+=(t=e)?1:-1;r+=this._getBadness(i)}f<0&&(f=-f);var c=0,u=f;for(u+=u<<2,u<<=1;u>h*h;)u-=h*h,c++;for(r+=c*v.N4,n=0;n<h;n++){for(i=0,o[0]=0,t=0,s=0;s<h;s++)t===(e=a[n+h*s])?o[i]++:o[++i]=1,t=e;r+=this._getBadness(i)}return r},_convertBitStream:function(t){var e,i,n=this._ecc,s=this._version;for(i=0;i<t;i++)n[i]=this._value.charCodeAt(i);var r=this._stringBuffer=n.slice(),o=this._calculateMaxLength();t>=o-2&&(t=o-2,s>9&&t--);var a=t;if(s>9){for(r[a+2]=0,r[a+3]=0;a--;)e=r[a],r[a+3]|=255&e<<4,r[a+2]=e>>4;r[2]|=255&t<<4,r[1]=t>>4,r[0]=64|t>>12}else{for(r[a+1]=0,r[a+2]=0;a--;)e=r[a],r[a+2]|=255&e<<4,r[a+1]=e>>4;r[1]|=255&t<<4,r[0]=64|t>>4}for(a=t+3-(s<10);a<o;)r[a++]=236,r[a++]=17},_getBadness:function(t){var e,i=0,n=this._badness;for(e=0;e<=t;e++)n[e]>=5&&(i+=v.N1+n[e]-5);for(e=3;e<t-1;e+=2)n[e-2]===n[e+2]&&n[e+2]===n[e-1]&&n[e-1]===n[e+1]&&3*n[e-1]===n[e]&&(0===n[e-3]||e+3>t||3*n[e-3]>=4*n[e]||3*n[e+3]>=4*n[e])&&(i+=v.N3);return i},_finish:function(){this._stringBuffer=this.buffer.slice();var t,e,i=0,n=3e4;for(e=0;e<8&&(this._applyMask(e),(t=this._checkBadness())<n&&(n=t,i=e),7!==i);e++)this.buffer=this._stringBuffer.slice();i!==e&&this._applyMask(i),n=l.FINAL_FORMAT[i+(this._level-1<<3)];var s=this.buffer,r=this.width;for(e=0;e<8;e++,n>>=1)1&n&&(s[r-1-e+8*r]=1,e<6?s[8+r*e]=1:s[8+r*(e+1)]=1);for(e=0;e<7;e++,n>>=1)1&n&&(s[8+r*(r-7+e)]=1,e?s[6-e+8*r]=1:s[7+8*r]=1)},_interleaveBlocks:function(){var t,e,i=this._dataBlock,n=this._ecc,s=this._eccBlock,r=0,o=this._calculateMaxLength(),a=this._neccBlock1,h=this._neccBlock2,f=this._stringBuffer;for(t=0;t<i;t++){for(e=0;e<a;e++)n[r++]=f[t+e*i];for(e=0;e<h;e++)n[r++]=f[a*i+t+e*(i+1)]}for(e=0;e<h;e++)n[r++]=f[a*i+t+e*(i+1)];for(t=0;t<s;t++)for(e=0;e<a+h;e++)n[r++]=f[o+t+e*s];this._stringBuffer=n},_insertAlignments:function(){var t,e,i,n=this._version,s=this.width;if(n>1)for(t=u.BLOCK[n],i=s-7;;){for(e=s-7;e>t-3&&(this._addAlignment(e,i),!(e<t));)e-=t;if(i<=t+9)break;i-=t,this._addAlignment(6,i),this._addAlignment(i,6)}},_insertFinders:function(){var t,e,i,n,s=this.buffer,r=this.width;for(t=0;t<3;t++){for(e=0,n=0,1===t&&(e=r-7),2===t&&(n=r-7),s[n+3+r*(e+3)]=1,i=0;i<6;i++)s[n+i+r*e]=1,s[n+r*(e+i+1)]=1,s[n+6+r*(e+i)]=1,s[n+i+1+r*(e+6)]=1;for(i=1;i<5;i++)this._setMask(n+i,e+1),this._setMask(n+1,e+i+1),this._setMask(n+5,e+i),this._setMask(n+i+1,e+5);for(i=2;i<4;i++)s[n+i+r*(e+2)]=1,s[n+2+r*(e+i+1)]=1,s[n+4+r*(e+i)]=1,s[n+i+1+r*(e+4)]=1}},_insertTimingGap:function(){var t,e,i=this.width;for(e=0;e<7;e++)this._setMask(7,e),this._setMask(i-8,e),this._setMask(7,e+i-7);for(t=0;t<8;t++)this._setMask(t,7),this._setMask(t+i-8,7),this._setMask(t,i-8)},_insertTimingRowAndColumn:function(){var t,e=this.buffer,i=this.width;for(t=0;t<i-14;t++)1&t?(this._setMask(8+t,6),this._setMask(6,8+t)):(e[8+t+6*i]=1,e[6+i*(8+t)]=1)},_insertVersion:function(){var t,e,i,n,s=this.buffer,r=this._version,o=this.width;if(r>6)for(t=d.BLOCK[r-7],e=17,i=0;i<6;i++)for(n=0;n<3;n++,e--)1&(e>11?r>>e-12:t>>e)?(s[5-i+o*(2-n+o-11)]=1,s[2-n+o-11+o*(5-i)]=1):(this._setMask(5-i,2-n+o-11),this._setMask(2-n+o-11,5-i))},_isMasked:function(t,e){var i=v._getMaskBit(t,e);return 1===this._mask[i]},_pack:function(){var t,e,i,n=1,s=1,r=this.width,o=r-1,a=r-1,h=(this._dataBlock+this._eccBlock)*(this._neccBlock1+this._neccBlock2)+this._neccBlock2;for(e=0;e<h;e++)for(t=this._stringBuffer[e],i=0;i<8;i++,t<<=1){128&t&&(this.buffer[o+r*a]=1);do{s?o--:(o++,n?0!==a?a--:(n=!n,6===(o-=2)&&(o--,a=9)):a!==r-1?a++:(n=!n,6===(o-=2)&&(o--,a-=8))),s=!s}while(this._isMasked(o,a))}},_reverseMask:function(){var t,e,i=this.width;for(t=0;t<9;t++)this._setMask(t,8);for(t=0;t<8;t++)this._setMask(t+i-8,8),this._setMask(8,t);for(e=0;e<7;e++)this._setMask(8,e+i-7)},_setMask:function(t,e){var i=v._getMaskBit(t,e);this._mask[i]=1},_syncMask:function(){var t,e,i=this.width;for(e=0;e<i;e++)for(t=0;t<=e;t++)this.buffer[t+i*e]&&this._setMask(t,e)}},{_createArray:function(t){var e,i=[];for(e=0;e<t;e++)i[e]=0;return i},_getMaskBit:function(t,e){var i;return t>e&&(i=t,t=e,e=i),i=e,i+=e*e,i>>=1,i+=t},_modN:function(t){for(;t>=255;)t=((t-=255)>>8)+(255&t);return t},N1:3,N2:3,N3:40,N4:10}),p=v,m=f.extend({draw:function(){this.element.src=this.qrious.toDataURL()},reset:function(){this.element.src=""},resize:function(){var t=this.element;t.width=t.height=this.qrious.size}}),g=h.extend(function(t,e,i,n){this.name=t,this.modifiable=Boolean(e),this.defaultValue=i,this._valueTransformer=n},{transform:function(t){var e=this._valueTransformer;return"function"==typeof e?e(t,this):t}}),k=h.extend(null,{abs:function(t){return null!=t?Math.abs(t):null},hasOwn:function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},noop:function(){},toUpperCase:function(t){return null!=t?t.toUpperCase():null}}),w=h.extend(function(t){this.options={},t.forEach(function(t){this.options[t.name]=t},this)},{exists:function(t){return null!=this.options[t]},get:function(t,e){return w._get(this.options[t],e)},getAll:function(t){var e,i=this.options,n={};for(e in i)k.hasOwn(i,e)&&(n[e]=w._get(i[e],t));return n},init:function(t,e,i){"function"!=typeof i&&(i=k.noop);var n,s;for(n in this.options)k.hasOwn(this.options,n)&&(s=this.options[n],w._set(s,s.defaultValue,e),w._createAccessor(s,e,i));this._setAll(t,e,!0)},set:function(t,e,i){return this._set(t,e,i)},setAll:function(t,e){return this._setAll(t,e)},_set:function(t,e,i,n){var s=this.options[t];if(!s)throw new Error("Invalid option: "+t);if(!s.modifiable&&!n)throw new Error("Option cannot be modified: "+t);return w._set(s,e,i)},_setAll:function(t,e,i){if(!t)return!1;var n,s=!1;for(n in t)k.hasOwn(t,n)&&this._set(n,t[n],e,i)&&(s=!0);return s}},{_createAccessor:function(t,e,i){var n={get:function(){return w._get(t,e)}};t.modifiable&&(n.set=function(n){w._set(t,n,e)&&i(n,t)}),Object.defineProperty(e,t.name,n)},_get:function(t,e){return e["_"+t.name]},_set:function(t,e,i){var n="_"+t.name,s=i[n],r=t.transform(null!=e?e:t.defaultValue);return i[n]=r,r!==s}}),M=w,b=h.extend(function(){this._services={}},{getService:function(t){var e=this._services[t];if(!e)throw new Error("Service is not being managed with name: "+t);return e},setService:function(t,e){if(this._services[t])throw new Error("Service is already managed with name: "+t);e&&(this._services[t]=e)}}),B=new M([new g("background",!0,"white"),new g("backgroundAlpha",!0,1,k.abs),new g("element"),new g("foreground",!0,"black"),new g("foregroundAlpha",!0,1,k.abs),new g("level",!0,"L",k.toUpperCase),new g("mime",!0,"image/png"),new g("padding",!0,null,k.abs),new g("size",!0,100,k.abs),new g("value",!0,"")]),y=new b,O=h.extend(function(t){B.init(t,this,this.update.bind(this));var e=B.get("element",this),i=y.getService("element"),n=e&&i.isCanvas(e)?e:i.createCanvas(),s=e&&i.isImage(e)?e:i.createImage();this._canvasRenderer=new c(this,n,!0),this._imageRenderer=new m(this,s,s===e),this.update()},{get:function(){return B.getAll(this)},set:function(t){B.setAll(t,this)&&this.update()},toDataURL:function(t){return this.canvas.toDataURL(t||this.mime)},update:function(){var t=new p({level:this.level,value:this.value});this._canvasRenderer.render(t),this._imageRenderer.render(t)}},{use:function(t){y.setService(t.getName(),t)}});Object.defineProperties(O.prototype,{canvas:{get:function(){return this._canvasRenderer.getElement()}},image:{get:function(){return this._imageRenderer.getElement()}}});var A=O,L=h.extend({getName:function(){}}).extend({createCanvas:function(){},createImage:function(){},getName:function(){return"element"},isCanvas:function(t){},isImage:function(t){}}).extend({createCanvas:function(){return document.createElement("canvas")},createImage:function(){return document.createElement("img")},isCanvas:function(t){return t instanceof HTMLCanvasElement},isImage:function(t){return t instanceof HTMLImageElement}});return A.use(new L),A});

// Global Remote Debugging Overlay (v1.1.9)
window.addEventListener("error", (e) => {
    showDebugError(`Uncaught Error: ${e.message} at ${e.filename}:${e.lineno}`);
});
window.addEventListener("unhandledrejection", (e) => {
    showDebugError(`Unhandled Rejection: ${e.reason}`);
});

function showDebugError(msg) {
    let debugDiv = document.getElementById("sonar-debug-overlay");
    if (!debugDiv) {
        debugDiv = document.createElement("div");
        debugDiv.id = "sonar-debug-overlay";
        debugDiv.style.cssText = "position:fixed; bottom:80px; right:20px; background:rgba(239,68,68,0.95); color:white; padding:12px 16px; border-radius:8px; z-index:99999; max-width:400px; font-family:monospace; font-size:11px; box-shadow:0 10px 25px rgba(0,0,0,0.5); pointer-events:none; word-wrap:break-word;";
        document.body.appendChild(debugDiv);
    }
    debugDiv.innerHTML += `<div style="margin-bottom:6px; border-bottom:1px solid rgba(255,255,255,0.2); padding-bottom:4px;">${msg}</div>`;
}

// Unregister Service Worker to prevent caching interference on local server
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.getRegistrations().then(registrations => {
            for (let reg of registrations) {
                reg.unregister();
            }
        }).catch(() => {});
    });
}
window.FALLBACK_ART_SVG = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='%23475569'%3E%3Cpath d='M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14.5c-2.49 0-4.5-2.01-4.5-4.5S9.51 7.5 12 7.5s4.5 2.01 4.5 4.5-2.01 4.5-4.5 4.5zm0-5.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z'/%3E%3C/svg%3E";
window.handleArtError = function(img) {
    if (img) {
        img.onerror = null;
        img.src = window.FALLBACK_ART_SVG;
    }
};

// --- Non-Blocking Toast Notification Engine (Replaces browser alert popups) ---
function showToast(message, type = "info") {
    let toastContainer = document.getElementById("toast-notification-container");
    if (!toastContainer) {
        toastContainer = document.createElement("div");
        toastContainer.id = "toast-notification-container";
        toastContainer.style.cssText = "position: fixed; bottom: 85px; right: 24px; z-index: 99999; display: flex; flex-direction: column; gap: 8px; pointer-events: none; font-family: 'Plus Jakarta Sans', sans-serif;";
        document.body.appendChild(toastContainer);
    }

    const toast = document.createElement("div");
    const bgColor = type === "error" ? "rgba(239, 68, 68, 0.95)" : (type === "success" ? "rgba(16, 185, 129, 0.95)" : "rgba(15, 23, 42, 0.95)");
    const borderColor = type === "error" ? "#f87171" : (type === "success" ? "#34d399" : "#06b6d4");
    const icon = type === "error" ? "fa-triangle-exclamation" : (type === "success" ? "fa-circle-check" : "fa-circle-info");

    toast.style.cssText = `background: ${bgColor}; color: #ffffff; border-left: 4px solid ${borderColor}; padding: 10px 16px; border-radius: 8px; font-size: 13px; font-weight: 600; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4); display: flex; align-items: center; gap: 10px; opacity: 0; transform: translateY(12px); transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); pointer-events: auto; backdrop-filter: blur(12px);`;
    toast.innerHTML = `<i class="fa-solid ${icon}" style="color: ${borderColor}; font-size: 15px;"></i> <span>${message}</span>`;

    toastContainer.appendChild(toast);

    requestAnimationFrame(() => {
        toast.style.opacity = "1";
        toast.style.transform = "translateY(0)";
    });

    setTimeout(() => {
        toast.style.opacity = "0";
        toast.style.transform = "translateY(-10px)";
        setTimeout(() => {
            if (toast.parentNode) toast.parentNode.removeChild(toast);
        }, 300);
    }, 2500);
}

// Override window.alert to route all popups silently into sleek toasts!
window.alert = function(msg) {
    if (msg) showToast(String(msg));
};

// Live Telemetry status updates are streamed via native WebSockets (/ws)

function escapeHtml(str) {
    if (str === null || str === undefined) return "";
    return String(str)
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
}

let libraryViewMode = "list";
let customEqPresets = {};
let lastSyncedQueueStr = "";
let currentEq10Band = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
let eqSendTimeout = null;

const EQ_FREQUENCIES = ["31Hz", "63Hz", "125Hz", "250Hz", "500Hz", "1kHz", "2kHz", "4kHz", "8kHz", "16kHz"];
const EQ_PRESETS = {
    flat: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    bass: [6, 5, 4, 2, 0, 0, 0, 0, 0, 0],
    vocal: [-2, -1, 0, 1, 3, 4, 3, 1, 0, -1],
    treble: [0, 0, 0, 0, 0, 1, 2, 4, 5, 6],
    electronic: [5, 4, 2, 0, -1, 1, 3, 4, 4, 3],
    rock: [4, 3, 1, 0, -1, 0, 2, 3, 4, 4],
    pop: [-1, 1, 3, 4, 4, 3, 1, 0, 1, 2],
    acoustic: [2, 1, 1, 0, 1, 2, 3, 3, 2, 1],
    classical: [3, 2, 1, 0, 0, 0, -1, -1, -2, -3]
};

// Application State
let state = {
    searchQuery: "",
    vocalFilter: "",
    characterFilter: "",
    sortBy: "title",
    sortOrder: "asc",
    currentPage: 1,
    limit: 50,
    totalPages: 1,
    activeTrackId: null,
    isPlaying: false,
    localPlayTimeSec: 0.0,
    lastProgressUpdateTime: 0.0,
    lyricLines: [], // parsed [{time: seconds, text: "string"}]
    activePlaylist: [], // list of ALL matching track objects for global queue playback
    currentAlbumTracks: [], // cached track objects for the current track's album
    repeatMode: "all", // "none", "all", "album", "artist", "one"
    lastRepeatMode: "all", // cached repeat style: "all", "album", "artist", "one"
    shuffleMode: false,
    lastShuffleMode: "normal", // cached smart shuffle mode: "normal", "ai", "melody"
    volumeKeysTarget: "exclusive", // "exclusive" (WASAPI Music Player) or "null" (System / Null Device)
    shuffleIndices: [], // shuffled order list
    selectedTrackIds: new Set(), // Selected track IDs for M3U playlist creator
    activeWorkspace: "workspace-library", // active center workspace
    queueVersion: -1,
    keyFilter: "",
    scaleFilter: "",
    emotionFilter: "",
    stringsFilter: "",
    keyboardsFilter: "",
    pianoFilter: "",
    drumsFilter: "",
    complexityFilter: "",
    choirFilter: "",
    bassFilter: "",
    guitarFilter: "",
    windsFilter: "",
    synthFilter: "",
    brassFilter: "",
    dreaminessFilter: "",
    epicnessFilter: "",
    cinematicnessFilter: "",
    electronicnessFilter: "",
    nostalgiaFilter: "",
    bpmFilter: "",
    // Playlist Builder filters
    pbSearchQuery: "",
    pbVocalFilter: "",
    pbCharacterFilter: "",
    pbKeyFilter: "",
    pbScaleFilter: "",
    pbEmotionFilter: "",
    pbStringsFilter: "",
    pbKeyboardsFilter: "",
    pbPianoFilter: "",
    pbDrumsFilter: "",
    pbComplexityFilter: "",
    pbChoirFilter: "",
    pbBassFilter: "",
    pbGuitarFilter: "",
    pbWindsFilter: "",
    pbSynthFilter: "",
    pbBrassFilter: "",
    pbDreaminessFilter: "",
    pbEpicnessFilter: "",
    pbCinematicnessFilter: "",
    pbElectronicnessFilter: "",
    pbNostalgiaFilter: "",
    pbBpmFilter: "",
    serverSettings: null
};

let userStateBuffer = {};
let saveStateTimeout = null;

function saveServerState(key, value) {
    if (!window.serverState) window.serverState = { preferences: {} };
    if (!window.serverState.preferences) window.serverState.preferences = {};
    window.serverState.preferences[key] = value;

    userStateBuffer[key] = value;
    if (saveStateTimeout) clearTimeout(saveStateTimeout);
    saveStateTimeout = setTimeout(async () => {
        await flushServerState();
    }, 1000);
}

async function flushServerState() {
    if (saveStateTimeout) {
        clearTimeout(saveStateTimeout);
        saveStateTimeout = null;
    }
    const keys = Object.keys(userStateBuffer);
    if (keys.length === 0) return;
    const payload = { ...userStateBuffer };
    userStateBuffer = {};
    try {
        await fetch("/api/user_state", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
    } catch (e) {
        console.error("Error saving/flushing server state:", e);
    }
}

// Visual Themes Configuration (v1.3.0)
const themesList = [
    {
        id: "system", name: "System Default", type: "system", icon: "fa-desktop",
        desc: "Automatically adapts to your operating system's light or dark preference.",
        preview: ["#090d16", "#ffffff", "#8b5cf6", "#22d3ee", "#34d399"],
        gradient: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
        tag: "Auto"
    },
    {
        id: "sonar-dark", name: "Sonar Dark", type: "dark", icon: "fa-moon",
        desc: "The signature deep space aesthetic with purple neon accents and frosted glass.",
        preview: ["#030509", "#f8fafc", "#c084fc", "#22d3ee", "#34d399"],
        gradient: "linear-gradient(135deg, #0c0a1a 0%, #1a0533 50%, #030509 100%)",
        tag: "Default"
    },
    {
        id: "alabaster-light", name: "Alabaster Light", type: "light", icon: "fa-sun",
        desc: "Warm premium off-white with soft violet borders and charcoal typography.",
        preview: ["#f9fafb", "#1c1917", "#8b5cf6", "#f59e0b", "#10b981"],
        gradient: "linear-gradient(135deg, #f5f7fa 0%, #e8e0f0 50%, #faf5ff 100%)",
        tag: "Light"
    },
    {
        id: "cyberpunk", name: "Cyberpunk Neon", type: "dark", icon: "fa-bolt",
        desc: "High-voltage electric pink and cyan on pitch-black with glitch energy.",
        preview: ["#030008", "#ffffff", "#ff007f", "#00f0ff", "#39ff14"],
        gradient: "linear-gradient(135deg, #030008 0%, #1a0025 30%, #001a1f 70%, #030008 100%)",
        tag: "Intense"
    },
    {
        id: "emerald-forest", name: "Emerald Forest", type: "dark", icon: "fa-leaf",
        desc: "Deep woodland greens with golden amber highlights and organic warmth.",
        preview: ["#020c06", "#ecfdf5", "#10b981", "#34d399", "#f59e0b"],
        gradient: "linear-gradient(135deg, #020c06 0%, #052e16 50%, #0a1f10 100%)",
        tag: "Nature"
    },
    {
        id: "deep-ocean", name: "Deep Ocean", type: "dark", icon: "fa-water",
        desc: "Abyssal blue depths with bioluminescent cyan and teal accents.",
        preview: ["#020817", "#e2e8f0", "#38bdf8", "#06b6d4", "#64ffda"],
        gradient: "linear-gradient(135deg, #020817 0%, #0a1929 50%, #051525 100%)",
        tag: "Calm"
    },
    {
        id: "sunset-gold", name: "Sunset Gold", type: "dark", icon: "fa-sun",
        desc: "Volcanic amber glow with crimson warmth and desert night skies.",
        preview: ["#0f0702", "#fff7ed", "#f97316", "#f59e0b", "#ef4444"],
        gradient: "linear-gradient(135deg, #0f0702 0%, #2a1505 50%, #1a0a02 100%)",
        tag: "Warm"
    },
    {
        id: "amethyst-glass", name: "Amethyst Glass", type: "dark", icon: "fa-gem",
        desc: "Royal violet crystals with prismatic pink refractions and deep plum.",
        preview: ["#080313", "#faf5ff", "#c084fc", "#e879f9", "#a855f7"],
        gradient: "linear-gradient(135deg, #080313 0%, #1a0730 50%, #0d0420 100%)",
        tag: "Elegant"
    },
    {
        id: "dracula", name: "Dracula Midnight", type: "dark", icon: "fa-ghost",
        desc: "The classic vampire IDE palette — muted purples, greens, and soft pinks.",
        preview: ["#1e1f29", "#f8f8f2", "#bd93f9", "#8be9fd", "#50fa7b"],
        gradient: "linear-gradient(135deg, #1e1f29 0%, #282a36 50%, #22232e 100%)",
        tag: "Classic"
    },
    {
        id: "nord", name: "Nord Frost", type: "dark", icon: "fa-snowflake",
        desc: "Arctic polar palette inspired by Nordic fjords with cool slate blues.",
        preview: ["#2e3440", "#eceff4", "#81a1c1", "#88c0d0", "#a3be8c"],
        gradient: "linear-gradient(135deg, #2e3440 0%, #3b4252 50%, #2e3440 100%)",
        tag: "Minimal"
    },
    {
        id: "sakura", name: "Sakura Blossom", type: "light", icon: "fa-fan",
        desc: "Cherry blossom petals floating on warm rose-tinted morning light.",
        preview: ["#fff5f6", "#4c0519", "#db2777", "#ec4899", "#f43f5e"],
        gradient: "linear-gradient(135deg, #fff5f6 0%, #ffe4e8 50%, #fff0f3 100%)",
        tag: "Soft"
    },
    {
        id: "crimson-console", name: "Crimson Console", type: "dark", icon: "fa-gamepad",
        desc: "Deep Charcoal-Plum gaming deck with glowing Crimson-Rose accents and silver labels.",
        preview: ["#160f16", "#f7f2f6", "#ff3a5c", "#ff7a90", "#ff2e55"],
        gradient: "linear-gradient(135deg, #160f16 0%, #231621 50%, #1c121a 100%)",
        tag: "Premium"
    }
];

function applyTheme(themeId) {
    // Remove all previous theme classes from body
    themesList.forEach(t => {
        if (t.id !== "system") {
            document.body.classList.remove(`theme-${t.id}`);
        }
    });

    let isLight = false;
    let actualTheme = themeId;

    if (themeId === "system") {
        const isSystemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        actualTheme = isSystemDark ? "sonar-dark" : "alabaster-light";
    }

    // Find theme type (light or dark)
    const activeThemeObj = themesList.find(t => t.id === actualTheme) || themesList[1];
    isLight = activeThemeObj.type === "light";

    if (actualTheme !== "sonar-dark") {
        document.body.classList.add(`theme-${actualTheme}`);
    }

    if (isLight) {
        document.body.classList.add("light-theme");
    } else {
        document.body.classList.remove("light-theme");
    }

    // Sync theme toggle button icon in sidebar
    const btnThemeToggle = document.getElementById("btn-theme-toggle");
    if (btnThemeToggle) {
        const icon = btnThemeToggle.querySelector("i");
        if (icon) {
            icon.className = isLight ? "fa-solid fa-sun" : "fa-solid fa-moon";
        }
    }

    // Save selection to localStorage
    saveServerState("player-theme-id", themeId);

    // Highlight active card in grid
    const cards = document.querySelectorAll(".theme-card");
    cards.forEach(c => {
        if (c.getAttribute("data-theme-id") === themeId) {
            c.classList.add("active-theme");
        } else {
            c.classList.remove("active-theme");
        }
    });
}

function loadVisualThemesWorkspace() {
    const wrapper = document.querySelector(".themes-tab-container");
    if (!wrapper) return;

    wrapper.innerHTML = "";

    const savedThemeId = (window.serverState?.preferences?.["player-theme-id"]) || "system";
    const activeTheme = themesList.find(t => t.id === savedThemeId) || themesList[0];

    // ── Hero Section ──
    const hero = document.createElement("div");
    hero.className = "vt-hero";
    hero.innerHTML = `
        <div class="vt-hero-glow"></div>
        <div class="vt-hero-content">
            <div class="vt-hero-badge"><i class="fa-solid fa-palette"></i> PERSONALIZATION</div>
            <h2 class="vt-hero-title">Visual Themes</h2>
            <p class="vt-hero-subtitle">Transform your workspace with handcrafted color palettes. Each theme redesigns every surface, accent, and glow across the entire interface.</p>
            <div class="vt-hero-active">
                <span class="vt-hero-active-label">Currently Active</span>
                <span class="vt-hero-active-name"><i class="fa-solid ${activeTheme.icon}"></i> ${activeTheme.name}</span>
            </div>
        </div>
    `;
    wrapper.appendChild(hero);

    // ── Theme Grid ──
    const grid = document.createElement("div");
    grid.className = "themes-grid";
    grid.id = "themes-grid-container";

    themesList.forEach((t, idx) => {
        const isActive = t.id === savedThemeId;
        const isLightTheme = t.type === "light";
        const borderCol = isLightTheme ? "rgba(0,0,0,0.06)" : "rgba(255,255,255,0.06)";
        const sidebarBg = isLightTheme
            ? `linear-gradient(180deg, ${t.preview[0]}dd, ${t.preview[0]})`
            : `linear-gradient(180deg, ${t.preview[0]}, ${t.preview[0]}ee)`;

        const card = document.createElement("div");
        card.className = `theme-card${isActive ? " active-theme" : ""}`;
        card.setAttribute("data-theme-id", t.id);
        card.style.animationDelay = `${idx * 60}ms`;

        card.innerHTML = `
            <div class="tc-mockup" style="background: ${t.gradient};">
                <div class="tc-mockup-inner">
                    <div class="tc-mock-sidebar" style="background: ${sidebarBg};">
                        <div class="tc-mock-nav-dot" style="background: ${t.preview[2]};"></div>
                        <div class="tc-mock-nav-dot" style="background: ${t.preview[3] || t.preview[2]}; opacity:0.5;"></div>
                        <div class="tc-mock-nav-dot" style="background: ${t.preview[4] || t.preview[2]}; opacity:0.3;"></div>
                    </div>
                    <div class="tc-mock-main">
                        <div class="tc-mock-header" style="background: ${t.preview[0]}cc; border-bottom: 1px solid ${borderCol};">
                            <div class="tc-mock-search" style="background: ${isLightTheme ? 'rgba(0,0,0,0.04)' : 'rgba(255,255,255,0.06)'}; border: 1px solid ${borderCol};"></div>
                        </div>
                        <div class="tc-mock-content" style="background: ${t.preview[0]};">
                            <div class="tc-mock-row" style="border-bottom: 1px solid ${borderCol};">
                                <div class="tc-mock-cell tc-w1" style="background: ${t.preview[2]}33;"></div>
                                <div class="tc-mock-cell tc-w2" style="background: ${t.preview[1]}18;"></div>
                                <div class="tc-mock-cell tc-w3" style="background: ${t.preview[3] || t.preview[2]}22;"></div>
                            </div>
                            <div class="tc-mock-row" style="border-bottom: 1px solid ${borderCol};">
                                <div class="tc-mock-cell tc-w1" style="background: ${t.preview[1]}12;"></div>
                                <div class="tc-mock-cell tc-w2" style="background: ${t.preview[2]}22;"></div>
                                <div class="tc-mock-cell tc-w3" style="background: ${t.preview[4] || t.preview[2]}18;"></div>
                            </div>
                            <div class="tc-mock-row">
                                <div class="tc-mock-cell tc-w1" style="background: ${t.preview[3] || t.preview[2]}18;"></div>
                                <div class="tc-mock-cell tc-w2" style="background: ${t.preview[1]}0d;"></div>
                                <div class="tc-mock-cell tc-w3" style="background: ${t.preview[2]}18;"></div>
                            </div>
                        </div>
                        <div class="tc-mock-player" style="background: ${t.preview[0]}ee; border-top: 1px solid ${borderCol};">
                            <div class="tc-mock-progress" style="background: ${t.preview[2]};"></div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="tc-palette-strip">
                ${t.preview.map(c => `<div class="tc-swatch" style="background:${c};"></div>`).join("")}
            </div>
            <div class="tc-body">
                <div class="tc-title-row">
                    <div class="tc-icon-wrap" style="background: ${t.preview[2]}22; color: ${t.preview[2]};">
                        <i class="fa-solid ${t.icon}"></i>
                    </div>
                    <div class="tc-title-col">
                        <span class="tc-name">${t.name}</span>
                        <span class="tc-type-badge tc-type-${t.type}">${t.tag || t.type}</span>
                    </div>
                </div>
                <p class="tc-desc">${t.desc}</p>
            </div>
            ${isActive ? '<div class="tc-active-badge"><i class="fa-solid fa-check"></i> Active</div>' : '<div class="tc-apply-hint">Click to apply</div>'}
        `;

        card.addEventListener("click", () => {
            applyTheme(t.id);
            // Re-render to update hero + active states
            loadVisualThemesWorkspace();
        });

        grid.appendChild(card);
    });

    wrapper.appendChild(grid);
}

// Register prefers-color-scheme listener
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    const savedThemeId = (window.serverState?.preferences?.["player-theme-id"]) || "system";
    if (savedThemeId === "system") {
        applyTheme("system");
    }
});

// Caching Layer (v1.1.0)
let trackDetailsCache = {};
let trackDetailsTimestamp = {};
let themeVariationCache = {};
let svgCache = {};

// Filter change tracking
let lastSearchQuery = null;
let lastVocalFilter = null;
let lastCharacterFilter = null;
let lastSortBy = null;
let lastSortOrder = null;
let lastKeyFilter = null;
let lastScaleFilter = null;
let lastEmotionFilter = null;
let lastStringsFilter = null;
let lastPianoFilter = null;
let lastDrumsFilter = null;
let lastComplexityFilter = null;
let lastChoirFilter = null;
let lastBassFilter = null;
let lastGuitarFilter = null;
let lastWindsFilter = null;
let lastSynthFilter = null;
let lastBrassFilter = null;
let lastDreaminessFilter = null;
let lastEpicnessFilter = null;
let lastCinematicnessFilter = null;
let lastElectronicnessFilter = null;
let lastNostalgiaFilter = null;
let lastBpmFilter = null;

// Web Audio DSP effects variables
let audioCtx = null;
let sourceNode = null;
let dspEnabled = true;

// Web Audio Nodes
let bassNode = null;
let eqNode = null;
let vocalsNode = null;
let airNode = null;
let compressorNode = null;
let limiterNode = null;
let warmthNode = null;
let reverbNode = null;
let reverbDryNode = null;
let reverbWetNode = null;
let reverbDryMix = null;
let reverbWetMix = null;
let msSplitter = null;
let msMerger = null;

function updateDSPBypassUI(bypass) {
    const globalSwitch = document.getElementById("dsp-global-enable");
    const globalText = document.getElementById("dsp-global-text");
    const controlBtn = document.getElementById("audio-btn-dsp-ab");

    if (globalSwitch) globalSwitch.checked = !bypass;
    if (globalText) {
        globalText.textContent = bypass ? "DSP OFF" : "DSP ON";
    }
    if (controlBtn) {
        if (bypass) {
            controlBtn.classList.remove("dsp-active");
            controlBtn.textContent = "DSP OFF";
        } else {
            controlBtn.classList.add("dsp-active");
            controlBtn.textContent = "DSP ON";
        }
    }
}

async function setDSPBypass(bypass) {
    updateDSPBypassUI(bypass);
    saveServerState("dsp-bypass", bypass);
    try {
        await fetch("/api/player/dsp", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ bypass: bypass })
        });
    } catch (e) {
        console.error("Error setting DSP bypass:", e);
    }
}
let midSum = null;
let sideDiff = null;
let invertGain = null;
let midGainNode = null;
let sideGainNode = null;
let sideInvert = null;
let crossfeedSplitter = null;
let crossfeedMerger = null;
let cfDirectL = null;
let cfDirectR = null;
let cfCrossL = null;
let cfCrossR = null;
let cfFilterL = null;
let cfFilterR = null;
let cfGainL = null;
let cfGainR = null;


// UI Elements (Initialized in DOMContentLoaded)
let searchInput, searchClearBtn, filterTags, filterCharacter, sortColumn, btnSortOrder;
let tracksTbody, paginationInfo, paginationCurrent, paginationFirst, paginationPrev, paginationNext, paginationLast;
let btnClearFilters, detailsDrawer, btnCloseDrawer, drawerEmpty, drawerInner;
let filterMusicalKey, filterMajorMinor, filterEmotion, filterStrings, filterPiano, filterDrums, filterComplexity;
let filterChoir, filterBass, filterGuitar, filterWinds, filterSynth, filterBrass, filterDreaminess, filterEpicness;
let filterCinematicness, filterElectronicness, filterNostalgia, filterBpm;

// Audio Player UI Elements
let audio, playPauseBtn, rewindBtn, forwardBtn, audioSlider, audioTimeCurrent, audioTimeTotal, volumeSlider, volumeIcon;

function initUiElements() {
    searchInput = document.getElementById("search-input");
    searchClearBtn = document.getElementById("search-clear-btn");
    filterTags = document.querySelectorAll("[data-vocal]");
    filterCharacter = document.getElementById("filter-character");
    sortColumn = document.getElementById("sort-column");
    btnSortOrder = document.getElementById("btn-sort-order");
    tracksTbody = document.getElementById("tracks-tbody");
    paginationInfo = document.getElementById("pagination-info");
    paginationCurrent = document.getElementById("pagination-current");
    paginationFirst = document.getElementById("pagination-first");
    paginationPrev = document.getElementById("pagination-prev");
    paginationNext = document.getElementById("pagination-next");
    paginationLast = document.getElementById("pagination-last");
    btnClearFilters = document.getElementById("btn-clear-filters");
    detailsDrawer = document.getElementById("details-drawer");
    btnCloseDrawer = document.getElementById("btn-close-drawer");
    drawerEmpty = document.getElementById("drawer-empty");
    drawerInner = document.getElementById("drawer-inner");
    filterMusicalKey = document.getElementById("filter-musical-key");
    filterMajorMinor = document.getElementById("filter-major-minor");
    filterEmotion = document.getElementById("filter-emotion");
    filterStrings = document.getElementById("filter-strings");
    filterPiano = document.getElementById("filter-piano");
    filterDrums = document.getElementById("filter-drums");
    filterComplexity = document.getElementById("filter-complexity");
    filterChoir = document.getElementById("filter-choir");
    filterBass = document.getElementById("filter-bass");
    filterGuitar = document.getElementById("filter-guitar");
    filterWinds = document.getElementById("filter-winds");
    filterSynth = document.getElementById("filter-synth");
    filterBrass = document.getElementById("filter-brass");
    filterDreaminess = document.getElementById("filter-dreaminess");
    filterEpicness = document.getElementById("filter-epicness");
    filterCinematicness = document.getElementById("filter-cinematicness");
    filterElectronicness = document.getElementById("filter-electronicness");
    filterNostalgia = document.getElementById("filter-nostalgia");
    filterBpm = document.getElementById("filter-bpm");

    audio = document.getElementById("main-audio-element");
    playPauseBtn = document.getElementById("audio-btn-play-pause");
    rewindBtn = document.getElementById("audio-btn-prev");
    forwardBtn = document.getElementById("audio-btn-next");
    audioSlider = document.getElementById("audio-slider");
    audioTimeCurrent = document.getElementById("audio-time-current");
    audioTimeTotal = document.getElementById("audio-time-total");
    volumeSlider = document.getElementById("audio-volume-slider");
    volumeIcon = document.getElementById("audio-volume-icon");
}

function setAudioSliderProgress(percent) {
    const safePercent = Math.max(0, Math.min(100, Number(percent) || 0));
    const scaleFactor = safePercent / 100;
    if (audioSlider && !audioSlider.dataset.dragging) {
        audioSlider.value = safePercent;
    }
    const fillEl = document.getElementById("audio-slider-fill");
    if (fillEl) {
        fillEl.style.transform = `scaleX(${scaleFactor})`;
    }
    if (audioSlider) {
        audioSlider.style.setProperty("--progress", `${safePercent}%`);
    }
}

function updatePlayerFromStatus(status) {
    if (!status) return;
    if (status.volume !== undefined) {
        const volSlider = document.getElementById("audio-volume-slider");
        const miniVolSlider = document.getElementById("mini-audio-volume-slider");
        if (volSlider) {
            volSlider.value = status.volume;
            volSlider.style.setProperty("--progress", `${status.volume}%`);
            updateVolumeIcon(status.volume / 100);
        }
        if (miniVolSlider) {
            miniVolSlider.value = status.volume;
            miniVolSlider.style.setProperty("--progress", `${status.volume}%`);
        }
    }
    if (status.track_id) {
        state.activeTrackId = status.track_id;
    }
}

// Debounce Timer
let searchDebounceTimer = null;

async function populateAlbumFilterOptions() {
    const sel = document.getElementById("filter-album");
    if (!sel) return;

    try {
        const res = await fetch("/api/albums");
        if (res.ok) {
            const albums = await res.json();
            let html = `
                <option value="">Group By: None (Flat List)</option>
                <optgroup label="Group Library By">
                    <option value="group:album">💿 Group By Album</option>
                    <option value="group:artist">🎙️ Group By Artist</option>
                    <option value="group:genre">🎼 Group By Genre</option>
                    <option value="group:year">📅 Group By Year</option>
                    <option value="group:folder">📁 Group By Folder</option>
                    <option value="group:format">🎚️ Group By Audio Format</option>
                </optgroup>
                <optgroup label="Filter Specific Album (${albums.length})">`;
            albums.forEach(a => {
                const name = a.name || a.album || "";
                if (name) {
                    html += `<option value="${escapeHtml(name)}">${escapeHtml(name)} (${a.trackCount || 1})</option>`;
                }
            });
            html += `</optgroup>`;
            sel.innerHTML = html;
        }
    } catch (e) {
        console.error("Error populating album filter options:", e);
    }

    if (!sel.dataset.bound) {
        sel.dataset.bound = "true";
        sel.addEventListener("change", (e) => {
            const val = e.target.value;
            if (val.startsWith("group:")) {
                const groupType = val.split(":")[1];
                const labels = {
                    album: "Albums",
                    artist: "Artists",
                    genre: "Genres",
                    year: "Years",
                    folder: "Folders",
                    format: "Audio Formats"
                };
                showGroupExplorerMode(groupType, labels[groupType] || groupType, true);
            } else if (val) {
                state.activeGroupingField = "album";
                state.activeGroupingValue = val;
                state.currentPage = 1;
                pushNavHistoryState({
                    type: "library_search",
                    query: state.searchQuery || "",
                    page: 1,
                    groupingField: "album",
                    groupingValue: val
                });
                showNormalListMode(false);
                loadTracks();
            } else {
                delete state.activeGroupingField;
                delete state.activeGroupingValue;
                state.currentPage = 1;
                showNormalListMode(true);
                loadTracks();
            }
        });
    }
}

// Initialize Application
async function initMainApp() {
    initUiElements();
    // 1. Fetch user state and active player status from server
    let backendStatus = null;
    try {
        const [resState, resStatus] = await Promise.all([
            fetch("/api/user_state"),
            fetch("/api/player/status")
        ]);
        if (resState.ok) {
            window.serverState = await resState.json();
        }
        if (resStatus.ok) {
            backendStatus = await resStatus.json();
        }
    } catch (e) {
        console.error("Init fetch failed:", e);
    }
    if (!window.serverState) window.serverState = { preferences: {} };
    if (!window.serverState.preferences) window.serverState.preferences = {};
    const pref = window.serverState.preferences;

    // Load settings from server preferences first
    loadPlayerState(backendStatus);

    // Set up event listeners and players
    setupEventListeners();
    setupAudioPlayer();
    setupVolumeControls();
    setupDspPreampAndPresets();
    setupDragResizers();
    setupWorkspaceSwitching();
    setupResponsiveLayout();
    setupDSPHandlers();
    setupGroupingMenu();
    if (typeof syncShuffleModeUI === "function") syncShuffleModeUI();
    if (typeof syncRepeatModeUI === "function") syncRepeatModeUI();
    if (typeof populateAlbumFilterOptions === "function") {
        populateAlbumFilterOptions();
    }

    // Restore Workspace and Filters
    if (pref["player-active-workspace"]) {
        const wsId = pref["player-active-workspace"];
        if (wsId === "workspace-albums") {
            const btn = document.getElementById("btn-show-albums");
            if (btn) btn.click();
        } else {
            const btn = document.querySelector(`.nav-item[data-workspace="${wsId}"]`);
            if (btn) btn.click();
        }
    }
    if (pref["player-search-query"]) {
        state.searchQuery = pref["player-search-query"];
        const searchInput = document.getElementById("search-input");
        if (searchInput) {
            searchInput.value = state.searchQuery;
            updateSearchClearBtnVisibility();
        }
    } else {
        updateSearchClearBtnVisibility();
    }
    if (pref["player-page"]) state.currentPage = parseInt(pref["player-page"]);
    if (pref["player-limit"]) {
        state.limit = parseInt(pref["player-limit"]);
        const sel = document.getElementById("per-page-select");
        if (sel) sel.value = state.limit;
    }
    if (pref["player-sort-col"]) state.sortBy = pref["player-sort-col"];
    if (pref["player-sort-order"]) state.sortOrder = pref["player-sort-order"];
    if (sortColumn) sortColumn.value = state.sortBy;
    if (btnSortOrder) {
        btnSortOrder.setAttribute("data-order", state.sortOrder);
        btnSortOrder.innerHTML = state.sortOrder === "asc"
            ? `<i class="fa-solid fa-arrow-down-a-z"></i>`
            : `<i class="fa-solid fa-arrow-up-z-a"></i>`;
    }
    if (pref["player-library-view"]) libraryViewMode = pref["player-library-view"];

    const activeViewBtn = document.querySelector(`.view-toggle[data-view="${libraryViewMode}"]`);
    if (activeViewBtn) {
        document.querySelectorAll(".view-toggle").forEach(b => b.classList.remove("active"));
        activeViewBtn.classList.add("active");
    }

    // Restore Dropdown filters
    const filterKeys = [
        "vocalFilter", "characterFilter", "keyFilter", "scaleFilter", "emotionFilter", "stringsFilter", "keyboardsFilter", "pianoFilter", "drumsFilter", "complexityFilter", "choirFilter", "bassFilter", "guitarFilter", "windsFilter", "synthFilter", "brassFilter", "dreaminessFilter", "epicnessFilter", "cinematicnessFilter", "electronicnessFilter", "nostalgiaFilter", "bpmFilter",
        "pbCharacterFilter", "pbKeyFilter", "pbScaleFilter", "pbEmotionFilter", "pbStringsFilter", "pbKeyboardsFilter", "pbPianoFilter", "pbDrumsFilter", "pbComplexityFilter", "pbChoirFilter", "pbBassFilter", "pbGuitarFilter", "pbWindsFilter", "pbSynthFilter", "pbBrassFilter", "pbDreaminessFilter", "pbEpicnessFilter", "pbCinematicnessFilter", "pbElectronicnessFilter", "pbNostalgiaFilter", "pbBpmFilter", "pbVocalFilter", "pbSearchQuery"
    ];
    filterKeys.forEach(k => {
        if (pref["player-filter-" + k]) {
            state[k] = pref["player-filter-" + k];
            const drop = document.querySelector(`select[data-state="${k}"]`);
            if (drop) {
                drop.value = state[k];
            } else if (k === "vocalFilter") {
                document.querySelectorAll("[data-vocal]").forEach(t => {
                    if (t.getAttribute("data-vocal") === state[k]) t.classList.add("active");
                    else t.classList.remove("active");
                });
            } else if (k === "pbVocalFilter") {
                document.querySelectorAll("[data-pb-vocal]").forEach(t => {
                    if (t.getAttribute("data-pb-vocal") === state[k]) t.classList.add("active");
                    else t.classList.remove("active");
                });
            } else if (k === "pbSearchQuery") {
                const inp = document.getElementById("pb-search-input");
                if (inp) inp.value = state[k];
            }
        }
    });

    if (typeof window.updateAdvancedFiltersBadge === "function") {
        window.updateAdvancedFiltersBadge();
    }
    if (typeof window.updatePbAdvancedFiltersBadge === "function") {
        window.updatePbAdvancedFiltersBadge();
    }
    loadPlaylistBuilderTracks();

    // Fetch statistics and tracks using restored filters
    loadStats();
    loadTracks();

    // Start search placeholder rotation loop
    startSearchPlaceholderRotation();

    // Remote control & OS media session setups
    initRemoteControlUI();
    setupMediaKeysAndKeyboard();
    startRemoteCommandPolling();
}

if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initMainApp);
} else {
    initMainApp();
}

// Setup Workspace navigation
function switchWorkspace(targetWorkspace, targetSubtab = null, shouldPushState = true) {
    if (!targetWorkspace) return;

    // Handle special overlay & modal requests from header bar
    if (targetWorkspace === "workspace-lyrics" || targetWorkspace === "fullscreen-overlay") {
        if (typeof loadTrackLyrics === "function" && state.activeTrackId) loadTrackLyrics(state.activeTrackId);
        const overlay = document.getElementById("fullscreen-overlay");
        if (overlay) overlay.style.display = "flex";
        return;
    }
    if (targetWorkspace === "workspace-tag-editor" || targetWorkspace === "tag-editor-modal") {
        if (state.activeTrackId) {
            openTagEditor(state.activeTrackId);
        } else {
            alert("Select a track first or press Ctrl+E");
        }
        return;
    }

    const navItems = document.querySelectorAll(".sidebar-nav .nav-item, #btn-show-settings");
    const workspaces = document.querySelectorAll(".workspace-panel");

    navItems.forEach(nav => nav.classList.remove("active"));
    const activeItem = Array.from(navItems).find(i => i.getAttribute("data-workspace") === targetWorkspace);
    if (activeItem) activeItem.classList.add("active");

    workspaces.forEach(ws => {
        if (ws.id === targetWorkspace) {
            ws.classList.add("active");
            ws.style.display = "flex";
        } else {
            ws.classList.remove("active");
            ws.style.display = "none";
        }
    });

    state.activeWorkspace = targetWorkspace;

    // Highlight top MusicBee navigation bar buttons
    document.querySelectorAll(".mb-tab-btn").forEach(btn => {
        const onClickAttr = btn.getAttribute("onclick") || "";
        if (onClickAttr.includes(targetWorkspace)) {
            btn.style.background = "var(--accent-cyan, #06b6d4)";
            btn.style.color = "#0f172a";
            btn.style.fontWeight = "700";
        } else {
            btn.style.background = "transparent";
            btn.style.color = "#94a3b8";
            btn.style.fontWeight = "600";
        }
    });

    if (targetSubtab) {
        const subtabBtn = document.querySelector(`.fav-tab-btn[data-favtab="${targetSubtab}"]`);
        if (subtabBtn) subtabBtn.click();
    }

    // Trigger specific workspace loaders
    if (targetWorkspace === "workspace-stats") {
        if (typeof loadStatsWorkspace === "function") loadStatsWorkspace();
    } else if (targetWorkspace === "workspace-playlist-builder") {
        if (typeof loadPlaylistBuilderTracks === "function") loadPlaylistBuilderTracks();
    } else if (targetWorkspace === "workspace-themes") {
        if (typeof loadThemeExplorer === "function") loadThemeExplorer();
    } else if (targetWorkspace === "workspace-favorites") {
        if (typeof loadFavoritesWorkspace === "function") loadFavoritesWorkspace();
    } else if (targetWorkspace === "workspace-queue") {
        if (typeof loadQueueWorkspace === "function") loadQueueWorkspace();
    } else if (targetWorkspace === "workspace-visual-themes") {
        if (typeof loadVisualThemesWorkspace === "function") loadVisualThemesWorkspace();
    } else if (targetWorkspace === "workspace-albums") {
        if (typeof loadAlbumsWorkspace === "function") loadAlbumsWorkspace();
    }

    if (shouldPushState) {
        pushNavHistoryState({ type: "workspace", workspaceId: targetWorkspace, subtabId: targetSubtab });
    }
}

function setupWorkspaceSwitching() {
    const navItems = document.querySelectorAll(".sidebar-nav .nav-item, #btn-show-settings");

    navItems.forEach(item => {
        item.addEventListener("click", (e) => {
            e.preventDefault();
            const targetWorkspace = item.getAttribute("data-workspace");
            const targetSubtab = item.getAttribute("data-subtab");
            switchWorkspace(targetWorkspace, targetSubtab, true);
        });
    });

    // Bind collapsible headers in DSP effects list
    document.querySelectorAll(".dsp-effect-header").forEach(hdr => {
        hdr.addEventListener("click", () => {
            hdr.parentElement.classList.toggle("collapsed");
        });
    });
}

// Fetch Library Stats
async function loadStats() {
    try {
        const res = await fetch("/api/stats");
        if (!res.ok) throw new Error("Stats fetch failed");
        const stats = await res.json();

        // Update Sidebar stats info if needed, otherwise ignore if elements are gone
        const sideTotal = document.getElementById("side-total-tracks");
        if (sideTotal) sideTotal.textContent = stats.total_tracks;

    } catch (err) {
        console.error("Error loading stats:", err);
    }
}

// Render dynamic Library Stats workspace dashboard
async function loadStatsWorkspace() {
    const grid = document.getElementById("stats-dashboard-grid");
    if (!grid) return;

    grid.innerHTML = `<div style="padding: 20px; font-size:14px; color:var(--text-mid);"><i class="fa-solid fa-spinner fa-spin"></i> Fetching statistics dashboard...</div>`;

    try {
        const res = await fetch("/api/stats");
        if (!res.ok) throw new Error("Stats fetch failed");
        const stats = await res.json();

        const vocalRatio = stats.total_tracks > 0 ? Math.round((stats.vocal_tracks / stats.total_tracks) * 100) : 0;
        const orchestralPct = Math.round(stats.avg_orchestralness * 100);

        // 1. Render core stats cards
        grid.innerHTML = `
            <div class="card card-purple">
                <div class="card-header">
                    <span class="card-title">Library Size</span>
                    <i class="fa-solid fa-music icon-accent"></i>
                </div>
                <div class="card-value">${stats.total_tracks}</div>
                <div class="card-desc">Fully indexed audio tracks</div>
            </div>

            <div class="card card-cyan">
                <div class="card-header">
                    <span class="card-title">Vocal Ratio</span>
                    <i class="fa-solid fa-chart-pie icon-accent"></i>
                </div>
                <div class="card-value">${vocalRatio}%</div>
                <div class="card-desc">${stats.vocal_tracks} vocals / ${stats.bgm_tracks} BGMs</div>
            </div>

            <div class="card card-violet">
                <div class="card-header">
                    <span class="card-title">Library Smoothness</span>
                    <i class="fa-solid fa-sliders icon-accent"></i>
                </div>
                <div class="card-value">${stats.avg_smoothness.toFixed(3)}</div>
                <div class="card-desc">Average neural smoothness index</div>
            </div>

            <div class="card card-emerald">
                <div class="card-header">
                    <span class="card-title">Theme Families</span>
                    <i class="fa-solid fa-diagram-project icon-accent"></i>
                </div>
                <div class="card-value">${stats.theme_families_count || 0}</div>
                <div class="card-desc">Distinct soundtrack motif clusters</div>
            </div>

            <div class="card card-yellow">
                <div class="card-header">
                    <span class="card-title">Average Tempo</span>
                    <i class="fa-solid fa-gauge-high icon-accent"></i>
                </div>
                <div class="card-value">${Math.round(stats.avg_bpm)} <span style="font-size:12px; font-weight:500; color:var(--text-mid);">BPM</span></div>
                <div class="card-desc">Average tempo beats per minute</div>
            </div>

            <div class="card card-rose">
                <div class="card-header">
                    <span class="card-title">Orchestral Index</span>
                    <i class="fa-solid fa-guitar icon-accent"></i>
                </div>
                <div class="card-value">${orchestralPct}%</div>
                <div class="card-desc">Average orchestral vs synth ratio</div>
            </div>
        `;

        // 2. Render Cinematic highlight
        const cinematicBody = document.getElementById("stat-most-cinematic-body");
        if (cinematicBody && stats.most_cinematic) {
            cinematicBody.innerHTML = `
                <div class="track-title" style="cursor:pointer;" onclick="playImmediate('${stats.most_cinematic.id}')">${escapeHtml(stats.most_cinematic.title)}</div>
                <div class="track-artist">${escapeHtml(stats.most_cinematic.artist)}</div>
                <div class="score-badge" style="background: var(--accent-purple-glow); color: var(--accent-purple); border: 1px solid rgba(192, 132, 252, 0.2);">Cinematic Index: ${(stats.most_cinematic.score * 100).toFixed(0)}%</div>
            `;
        } else if (cinematicBody) {
            cinematicBody.innerHTML = `<span class="val-empty">None detected</span>`;
        }

        // 3. Render Dreamy highlight
        const dreamyBody = document.getElementById("stat-most-dreamy-body");
        if (dreamyBody && stats.most_dreamy) {
            dreamyBody.innerHTML = `
                <div class="track-title" style="cursor:pointer;" onclick="playImmediate('${stats.most_dreamy.id}')">${escapeHtml(stats.most_dreamy.title)}</div>
                <div class="track-artist">${escapeHtml(stats.most_dreamy.artist)}</div>
                <div class="score-badge" style="background: var(--accent-cyan-glow); color: var(--accent-cyan); border: 1px solid rgba(34, 211, 238, 0.2);">Dreamy Index: ${(stats.most_dreamy.score * 100).toFixed(0)}%</div>
            `;
        } else if (dreamyBody) {
            dreamyBody.innerHTML = `<span class="val-empty">None detected</span>`;
        }

        // 4. Render top artists chart bars
        const artistsBody = document.getElementById("stat-top-artists-body");
        if (artistsBody && stats.top_artists && stats.top_artists.length > 0) {
            let maxCount = Math.max(...stats.top_artists.map(a => a.count));
            let html = "";
            stats.top_artists.forEach(a => {
                const pct = maxCount > 0 ? (a.count / maxCount) * 100 : 0;
                html += `
                    <div class="artist-bar-row">
                        <div class="artist-bar-meta">
                            <span class="artist-bar-name">${renderArtistLinks(a.artist, false)}</span>
                            <span class="artist-bar-count">${a.count} tracks</span>
                        </div>
                        <div class="artist-bar-outer">
                            <div class="artist-bar-inner" style="width: ${pct}%"></div>
                        </div>
                    </div>
                `;
            });
            artistsBody.innerHTML = html;
        } else if (artistsBody) {
            artistsBody.innerHTML = `<span class="val-empty">No artists statistical data available.</span>`;
        }

        // 5. Render top emotions chart bars
        const emotionsBody = document.getElementById("stat-top-emotions-body");
        if (emotionsBody && stats.top_emotions && stats.top_emotions.length > 0) {
            let maxEmotion = Math.max(...stats.top_emotions.map(e => e.count));
            let htmlEmotions = "";
            stats.top_emotions.forEach(e => {
                const pct = maxEmotion > 0 ? (e.count / maxEmotion) * 100 : 0;
                htmlEmotions += `
                    <div class="artist-bar-row">
                        <div class="artist-bar-meta">
                            <span class="artist-bar-name" style="cursor:pointer;" onclick="filterByEmotion(decodeURIComponent('${escapeJsParam(e.emotion)}'))">${escapeHtml(e.emotion)}</span>
                            <span class="artist-bar-count">${e.count} tracks</span>
                        </div>
                        <div class="artist-bar-outer">
                            <div class="artist-bar-inner" style="width: ${pct}%; background: var(--accent-gradient);"></div>
                        </div>
                    </div>
                `;
            });
            emotionsBody.innerHTML = htmlEmotions;
        } else if (emotionsBody) {
            emotionsBody.innerHTML = `<span class="val-empty">No emotions statistical data available.</span>`;
        }

        // 6. Render top keys chart bars
        const keysBody = document.getElementById("stat-top-keys-body");
        if (keysBody && stats.top_keys && stats.top_keys.length > 0) {
            let maxKey = Math.max(...stats.top_keys.map(k => k.count));
            let htmlKeys = "";
            stats.top_keys.forEach(k => {
                const pct = maxKey > 0 ? (k.count / maxKey) * 100 : 0;
                htmlKeys += `
                    <div class="artist-bar-row">
                        <div class="artist-bar-meta">
                            <span class="artist-bar-name" style="cursor:pointer;" onclick="filterByKey(decodeURIComponent('${escapeJsParam(k.key)}'))">${escapeHtml(k.key)}</span>
                            <span class="artist-bar-count">${k.count} tracks</span>
                        </div>
                        <div class="artist-bar-outer">
                            <div class="artist-bar-inner" style="width: ${pct}%; background: linear-gradient(90deg, var(--accent-cyan), #06b6d4);"></div>
                        </div>
                    </div>
                `;
            });
            keysBody.innerHTML = htmlKeys;
        } else if (keysBody) {
            keysBody.innerHTML = `<span class="val-empty">No key signatures statistical data available.</span>`;
        }

        // 7. Render acoustic fingerprint averages
        const acousticBody = document.getElementById("stat-acoustic-profile-body");
        if (acousticBody) {
            const instrumentStats = [
                { name: "Strings Density", val: stats.avg_strings, color: "var(--accent-purple)" },
                { name: "Piano Presence", val: stats.avg_piano, color: "var(--accent-cyan)" },
                { name: "Choir Presence", val: stats.avg_choir, color: "var(--accent-rose)" },
                { name: "Bass Presence", val: stats.avg_bass, color: "var(--accent-yellow)" },
                { name: "Drums / Beats", val: stats.avg_drums, color: "var(--accent-emerald)" },
                { name: "Winds Presence", val: stats.avg_winds, color: "var(--accent-teal)" },
                { name: "Synth Presence", val: stats.avg_synth, color: "var(--accent-orange)" },
                { name: "Brass Presence", val: stats.avg_brass, color: "var(--accent-red)" }
            ];
            let htmlAcoustic = "";
            instrumentStats.forEach(inst => {
                const pct = inst.val * 100;
                htmlAcoustic += `
                    <div class="artist-bar-row">
                        <div class="artist-bar-meta">
                            <span class="artist-bar-name">${inst.name}</span>
                            <span class="artist-bar-count">${Math.round(pct)}% avg</span>
                        </div>
                        <div class="artist-bar-outer">
                            <div class="artist-bar-inner" style="width: ${pct}%; background: ${inst.color};"></div>
                        </div>
                    </div>
                `;
            });

            // Add Smoothness category counts
            const totalScanned = stats.distribution.calm + stats.distribution.moderate + stats.distribution.dynamic;
            if (totalScanned > 0) {
                htmlAcoustic += `
                    <div style="margin-top: 16px; margin-bottom: 8px; font-size: 11px; font-weight: 800; text-transform: uppercase; color: var(--text-low);">Smoothness Profiles</div>
                    <div class="artist-bar-row">
                        <div class="artist-bar-meta">
                            <span class="artist-bar-name">Calm / Smooth</span>
                            <span class="artist-bar-count">${stats.distribution.calm} tracks</span>
                        </div>
                        <div class="artist-bar-outer">
                            <div class="artist-bar-inner" style="width: ${(stats.distribution.calm / totalScanned) * 100}%; background: var(--accent-emerald);"></div>
                        </div>
                    </div>
                    <div class="artist-bar-row">
                        <div class="artist-bar-meta">
                            <span class="artist-bar-name">Moderate</span>
                            <span class="artist-bar-count">${stats.distribution.moderate} tracks</span>
                        </div>
                        <div class="artist-bar-outer">
                            <div class="artist-bar-inner" style="width: ${(stats.distribution.moderate / totalScanned) * 100}%; background: var(--accent-cyan);"></div>
                        </div>
                    </div>
                    <div class="artist-bar-row">
                        <div class="artist-bar-meta">
                            <span class="artist-bar-name">Dynamic / Fluctuating</span>
                            <span class="artist-bar-count">${stats.distribution.dynamic} tracks</span>
                        </div>
                        <div class="artist-bar-outer">
                            <div class="artist-bar-inner" style="width: ${(stats.distribution.dynamic / totalScanned) * 100}%; background: var(--accent-rose);"></div>
                        </div>
                    </div>
                `;
            }
            acousticBody.innerHTML = htmlAcoustic;
        }

    } catch (err) {
        console.error("Error drawing statistics page:", err);
    }
}

// Fetch and Render Tracks List (Handles global active queue fetch for shuffle/navigation)
async function loadTracks() {
    const tbody = document.getElementById("tracks-tbody");
    if (tbody) {
        tbody.innerHTML = `
            <tr>
                <td colspan="8" class="table-loading">
                    <i class="fa-solid fa-spinner fa-spin"></i> Fetching matching tracks...
                </td>
            </tr>
        `;
    }

    // Construct Query String for UI table page rendering
    const params = new URLSearchParams();
    const addParam = (key, val) => {
        if (val !== undefined && val !== null && val !== "" && val !== "undefined" && val !== "null") {
            params.append(key, val);
        }
    };

    addParam("search", state.searchQuery);
    addParam("vocal", state.vocalFilter);
    addParam("character", state.characterFilter);
    addParam("key", state.keyFilter);
    addParam("scale", state.scaleFilter);
    addParam("emotion", state.emotionFilter);
    addParam("strings", state.stringsFilter);
    addParam("keyboards", state.keyboardsFilter);
    addParam("piano", state.pianoFilter);
    addParam("drums", state.drumsFilter);
    addParam("complexity", state.complexityFilter);
    addParam("choir", state.choirFilter);
    addParam("guitar", state.guitarFilter);
    addParam("bass", state.bassFilter);
    addParam("winds", state.windsFilter);
    addParam("synth", state.synthFilter);
    addParam("brass", state.brassFilter);
    addParam("dreaminess", state.dreaminessFilter);
    addParam("epicness", state.epicnessFilter);
    addParam("cinematicness", state.cinematicnessFilter);
    addParam("electronicness", state.electronicnessFilter);
    addParam("nostalgia", state.nostalgiaFilter);
    addParam("bpm", state.bpmFilter);
    addParam("sort", state.sortBy || "title");
    addParam("order", state.sortOrder || "asc");
    addParam("page", state.currentPage || 1);
    addParam("limit", state.limit || 50);

    if (state.activeGroupingField && state.activeGroupingValue !== undefined && state.activeGroupingValue !== null) {
        addParam(state.activeGroupingField, state.activeGroupingValue);
    }

    try {
        const res = await fetch(`/api/tracks?${params.toString()}`);
        if (!res.ok) throw new Error("Tracks fetch failed");
        const data = await res.json();

        state.totalPages = data.pages;
        state.tracks = data.tracks;
        renderTracks(data.tracks);
        renderPagination(data.total, data.page, data.limit);
        saveServerState("player-page", state.currentPage);

        // Trigger a secondary large fetch to cache ALL matching track objects as our playback playlist queue.
        // Cache checking avoids re-fetching the full queue on simple pagination changes.
        const filtersChanged =
            state.searchQuery !== lastSearchQuery ||
            state.vocalFilter !== lastVocalFilter ||
            state.characterFilter !== lastCharacterFilter ||
            state.keyFilter !== lastKeyFilter ||
            state.scaleFilter !== lastScaleFilter ||
            state.emotionFilter !== lastEmotionFilter ||
            state.stringsFilter !== lastStringsFilter ||
            state.keyboardsFilter !== lastKeyboardsFilter ||
            state.pianoFilter !== lastPianoFilter ||
            state.drumsFilter !== lastDrumsFilter ||
            state.complexityFilter !== lastComplexityFilter ||
            state.choirFilter !== lastChoirFilter ||
            state.guitarFilter !== lastGuitarFilter ||
            state.bassFilter !== lastBassFilter ||
            state.windsFilter !== lastWindsFilter ||
            state.synthFilter !== lastSynthFilter ||
            state.brassFilter !== lastBrassFilter ||
            state.dreaminessFilter !== lastDreaminessFilter ||
            state.epicnessFilter !== lastEpicnessFilter ||
            state.cinematicnessFilter !== lastCinematicnessFilter ||
            state.electronicnessFilter !== lastElectronicnessFilter ||
            state.nostalgiaFilter !== lastNostalgiaFilter ||
            state.bpmFilter !== lastBpmFilter ||
            state.sortBy !== lastSortBy ||
            state.sortOrder !== lastSortOrder ||
            !state.activePlaylist || state.activePlaylist.length === 0;

        if (filtersChanged) {
            lastSearchQuery = state.searchQuery;
            lastVocalFilter = state.vocalFilter;
            lastCharacterFilter = state.characterFilter;
            lastKeyFilter = state.keyFilter;
            lastScaleFilter = state.scaleFilter;
            lastEmotionFilter = state.emotionFilter;
            lastStringsFilter = state.stringsFilter;
            lastKeyboardsFilter = state.keyboardsFilter;
            lastPianoFilter = state.pianoFilter;
            lastDrumsFilter = state.drumsFilter;
            lastComplexityFilter = state.complexityFilter;
            lastChoirFilter = state.choirFilter;
            lastGuitarFilter = state.guitarFilter;
            lastBassFilter = state.bassFilter;
            lastWindsFilter = state.windsFilter;
            lastSynthFilter = state.synthFilter;
            lastBrassFilter = state.brassFilter;
            lastDreaminessFilter = state.dreaminessFilter;
            lastEpicnessFilter = state.epicnessFilter;
            lastCinematicnessFilter = state.cinematicnessFilter;
            lastElectronicnessFilter = state.electronicnessFilter;
            lastNostalgiaFilter = state.nostalgiaFilter;
            lastBpmFilter = state.bpmFilter;
            lastSortBy = state.sortBy;
            lastSortOrder = state.sortOrder;

            const queueParams = new URLSearchParams({
                search: state.searchQuery,
                vocal: state.vocalFilter,
                character: state.characterFilter,
                key: state.keyFilter,
                scale: state.scaleFilter,
                emotion: state.emotionFilter,
                strings: state.stringsFilter,
                keyboards: state.keyboardsFilter,
                piano: state.pianoFilter,
                drums: state.drumsFilter,
                complexity: state.complexityFilter,
                choir: state.choirFilter,
                guitar: state.guitarFilter,
                bass: state.bassFilter,
                winds: state.windsFilter,
                synth: state.synthFilter,
                brass: state.brassFilter,
                dreaminess: state.dreaminessFilter,
                epicness: state.epicnessFilter,
                cinematicness: state.cinematicnessFilter,
                electronicness: state.electronicnessFilter,
                nostalgia: state.nostalgiaFilter,
                bpm: state.bpmFilter,
                sort: state.sortBy,
                order: state.sortOrder,
                page: 1,
                limit: 100000,
                minimal: "true" // lightweight minimal columns fetch
            });

            const qRes = await fetch(`/api/tracks?${queueParams.toString()}`);
            if (qRes.ok) {
                const qData = await qRes.json();
                state.activePlaylist = qData.tracks || [];
                generateShuffleIndices();
                updateQueueWidget();
            }
        }

    } catch (err) {
        console.error("Error loading tracks:", err);
        const tbody = document.getElementById("tracks-tbody");
        if (tbody) {
            tbody.innerHTML = `
                <tr>
                    <td colspan="8" class="table-empty">
                        <i class="fa-solid fa-circle-exclamation text-accent"></i> Failed to query music library: ${escapeHtml(err.message || String(err))}
                    </td>
                </tr>
            `;
        }
    }
}

function resetAllFilters() {
    state.searchQuery = "";
    state.vocalFilter = "";
    state.characterFilter = "";
    state.keyFilter = "";
    state.scaleFilter = "";
    state.emotionFilter = "";
    state.stringsFilter = "";
    state.keyboardsFilter = "";
    state.pianoFilter = "";
    state.drumsFilter = "";
    state.complexityFilter = "";
    state.choirFilter = "";
    state.guitarFilter = "";
    state.bassFilter = "";
    state.windsFilter = "";
    state.synthFilter = "";
    state.brassFilter = "";
    state.dreaminessFilter = "";
    state.epicnessFilter = "";
    state.cinematicnessFilter = "";
    state.electronicnessFilter = "";
    state.nostalgiaFilter = "";
    state.bpmFilter = "";
    state.currentPage = 1;
    delete state.activeGroupingField;
    delete state.activeGroupingValue;

    const searchInp = document.getElementById("search-input");
    if (searchInp) searchInp.value = "";

    document.querySelectorAll("select[data-state]").forEach(s => s.value = "");
    document.querySelectorAll("[data-vocal]").forEach(b => {
        if (b.getAttribute("data-vocal") === "") b.classList.add("active");
        else b.classList.remove("active");
    });

    const filterKeys = [
        "vocalFilter", "characterFilter", "keyFilter", "scaleFilter", "emotionFilter", "stringsFilter", "keyboardsFilter", "pianoFilter", "drumsFilter", "complexityFilter", "choirFilter", "bassFilter", "guitarFilter", "windsFilter", "synthFilter", "brassFilter", "dreaminessFilter", "epicnessFilter", "cinematicnessFilter", "electronicnessFilter", "nostalgiaFilter", "bpmFilter"
    ];
    filterKeys.forEach(k => saveServerState("player-filter-" + k, ""));
    saveServerState("player-search-query", "");

    showNormalListMode(true);
}
window.resetAllFilters = resetAllFilters;

// Render tracks list in the table body

function renderTracks(tracks) {
    const tracksGrid = document.getElementById("tracks-grid");
    const tracksTbody = document.getElementById("tracks-tbody");
    if (tracksTbody) tracksTbody.innerHTML = "";
    if (tracksGrid) tracksGrid.innerHTML = "";

    // Toggle between standard Table and Grid layouts
    const tracksTable = document.getElementById("tracks-table");
    if (tracksTable && tracksGrid) {
        if (libraryViewMode === "list") {
            tracksTable.style.display = "";
            tracksGrid.style.display = "none";
        } else {
            tracksTable.style.display = "none";
            tracksGrid.style.display = "grid";
            tracksGrid.style.gridTemplateColumns = libraryViewMode === "medium" ? "repeat(auto-fill, minmax(180px, 1fr))" : "repeat(auto-fill, minmax(280px, 1fr))";
            tracksGrid.style.gap = "20px";
        }
    }

    if (tracks.length === 0) {
        const noResultsHtml = `
            <div style="padding: 40px; text-align: center; color: var(--text-muted); display: flex; flex-direction: column; align-items: center; gap: 12px;">
                <i class="fa-solid fa-filter-circle-xmark" style="font-size: 32px; color: #818cf8;"></i>
                <div style="font-size: 15px; font-weight: 600; color: #e2e8f0;">No tracks match your current filters.</div>
                <button onclick="resetAllFilters()" style="background: rgba(99, 102, 241, 0.2); border: 1px solid rgba(99, 102, 241, 0.4); color: #a5b4fc; padding: 8px 18px; border-radius: 8px; font-weight: 700; cursor: pointer; display: flex; align-items: center; gap: 8px; transition: all 0.2s;">
                    <i class="fa-solid fa-rotate-left"></i> Reset All Filters
                </button>
            </div>
        `;
        if (libraryViewMode === "list") {
            tracksTbody.innerHTML = `<tr><td colspan="8" class="text-center">${noResultsHtml}</td></tr>`;
        } else {
            tracksGrid.innerHTML = `<div class="text-center" style="grid-column: 1 / -1;">${noResultsHtml}</div>`;
        }
        return;
    }

    tracks.forEach((t, idx) => {
        const dur = formatDuration(t.duration);
        const isActive = Number(t.id) === Number(state.activeTrackId) ? "active-row" : "";
        const artUrl = `/api/art?id=${t.album_art_id || t.id}`;
        const eqBars = isActive ? `<span class="now-playing-eq" style="display:inline-flex; margin-left:6px; vertical-align:middle; flex-shrink:0;"><span class="eq-bar"></span><span class="eq-bar"></span><span class="eq-bar"></span><span class="eq-bar"></span></span>` : '';

        if (libraryViewMode === "list") {
            const tr = document.createElement("tr");
            tr.className = isActive;
            tr.setAttribute("data-id", t.id);

            const trackNo = t.track_number || (idx + 1);
            const isLoved = t.favorite_count > 0 || t.is_favorite;
            const heartIcon = isLoved ? `<i class="fa-solid fa-heart" style="color: #f43f5e;"></i>` : `<i class="fa-regular fa-heart"></i>`;
            const formatStr = t.file_path && t.file_path.toLowerCase().endsWith('.flac') ? 'FLAC' : (t.file_path && t.file_path.toLowerCase().endsWith('.wav') ? 'WAV' : 'MP3');
            const yearStr = t.year || t.date || '';
            const genreStr = t.genre || 'Soundtrack';

            // 5-Star Interactive Rating Component
            const userRating = Math.round(Number(t.rating || t.user_affinity || 0));
            let starsHtml = `<span class="star-rating" data-id="${t.id}" title="Rate track">`;
            for (let s = 1; s <= 5; s++) {
                const filled = s <= userRating ? "filled" : "";
                starsHtml += `<span class="star ${filled}" data-star="${s}">★</span>`;
            }
            starsHtml += `</span>`;

            tr.innerHTML = `
                <td style="text-align: center; color: #888888; font-weight: 600;">${trackNo}</td>
                <td class="title-col" style="display:flex; align-items:center; gap:6px; font-weight:600;">${escapeHtml(t.title)}${eqBars}</td>
                <td>${renderArtistLinks(t.artist)}</td>
                <td style="color: #94a3b8;">${escapeHtml(t.album || '')}</td>
                <td style="color: #888888;">${escapeHtml(genreStr)}</td>
                <td style="text-align: center; color: #888888;">${yearStr}</td>
                <td style="text-align: center; font-weight: 500;">${dur}</td>
                <td style="text-align: center;"><span class="badge" style="background: rgba(255,255,255,0.06); padding: 1px 5px; border-radius: 3px; font-size: 10px; font-weight: 700; color: #007acc;">${formatStr}</span></td>
                <td style="text-align: center;" onclick="event.stopPropagation()">${starsHtml}</td>
                <td style="text-align: center;" onclick="event.stopPropagation()">
                    <button class="fav-heart-btn ${isLoved ? 'loved' : ''}" data-id="${t.id}" title="Toggle Favorite">${heartIcon}</button>
                </td>
                <td class="col-action" onclick="event.stopPropagation()" style="display:flex; align-items:center; gap:6px; justify-content:center;">
                    <button class="row-play-btn" title="Play Track" style="background:none; border:none; color:#007acc; cursor:pointer; font-size:12px;">
                        <i class="fa-solid fa-play"></i>
                    </button>
                    <button class="row-edit-tag-btn" title="Edit Metadata Tags (Ctrl+E)" style="background:none; border:none; color:#f59e0b; cursor:pointer; font-size:11px;">
                        <i class="fa-solid fa-tags"></i>
                    </button>
                </td>
            `;

            // Right click context menu
            tr.addEventListener("contextmenu", (e) => {
                e.preventDefault();
                openMusicBeeContextMenu(e, t);
            });

            // Bind Star Rating click
            tr.querySelectorAll(".star-rating .star").forEach(starEl => {
                starEl.addEventListener("click", async (e) => {
                    e.stopPropagation();
                    const starVal = Number(starEl.getAttribute("data-star"));
                    await setTrackRating(t.id, starVal);
                });
            });

            // Bind Favorite Heart click
            const heartBtn = tr.querySelector(".fav-heart-btn");
            if (heartBtn) {
                heartBtn.addEventListener("click", async (e) => {
                    e.stopPropagation();
                    await toggleFavorite(t.id, heartBtn);
                });
            }

            tr.addEventListener("click", async () => {
                state.activePlaylist = tracks;
                generateShuffleIndices();
                await selectTrack(t.id);
                fetch("/api/player/queue", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ queue: tracks.map(tr => tr.id), start_track_id: t.id })
                });
            });
            const playBtn = tr.querySelector(".row-play-btn");
            if (playBtn) {
                playBtn.addEventListener("click", async (e) => {
                    e.stopPropagation();
                    state.activePlaylist = tracks;
                    generateShuffleIndices();
                    await selectTrack(t.id);
                    fetch("/api/player/queue", {
                        method: "POST",
                        headers: { "Content-Type": "application/json" },
                        body: JSON.stringify({ queue: tracks.map(tr => tr.id), start_track_id: t.id })
                    });
                });
            }
            const editBtn = tr.querySelector(".row-edit-tag-btn");
            if (editBtn) {
                editBtn.addEventListener("click", (e) => {
                    e.stopPropagation();
                    openTagEditor(t.id);
                });
            }
            tracksTbody.appendChild(tr);
        } else {
            const card = document.createElement("div");
            card.className = "grid-card " + isActive;
            card.setAttribute("data-id", t.id);
            card.innerHTML = `
                <div style="position:relative;">
                    <img src="${artUrl}" class="grid-card-img" onerror="handleArtError(this)">
                </div>
                <div class="grid-card-title" title="${escapeHtml(t.title)}" style="display:flex; align-items:center; gap:4px;">${escapeHtml(t.title)}${eqBars}</div>
                <div class="grid-card-artist" title="${escapeHtml(t.artist)}">${renderArtistLinks(t.artist, false)}</div>
            `;
            card.addEventListener("click", async () => {
                state.activePlaylist = tracks;
                generateShuffleIndices();
                await selectTrack(t.id);
                fetch("/api/player/queue", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ queue: tracks.map(tr => tr.id), start_track_id: t.id })
                });
            });
            tracksGrid.appendChild(card);
        }
    });
}


// Toggle Favorite status in database and sync UI
async function toggleFavorite(trackId, buttonElement) {
    try {
        const res = await fetch(`/api/interact?id=${trackId}&type=favorite`);
        if (!res.ok) throw new Error("Favorite API failed");
        const data = await res.json();

        const isFav = data.favorite_count > 0;

        // Update all star buttons for this trackId in UI
        const starBtns = document.querySelectorAll(`button.btn-favorite[data-id="${trackId}"]`);
        starBtns.forEach(btn => {
            const icon = btn.querySelector("i");
            if (isFav) {
                btn.classList.add("favorited");
                if (icon) icon.className = "fa-solid fa-star";
            } else {
                btn.classList.remove("favorited");
                if (icon) icon.className = "fa-regular fa-star";
            }
        });

        // Sync drawer star button if active
        if (state.activeTrackId === trackId) {
            const drawerStar = document.getElementById("btn-drawer-favorite");
            if (drawerStar) {
                const icon = drawerStar.querySelector("i");
                if (isFav) {
                    drawerStar.classList.add("favorited");
                    if (icon) icon.className = "fa-solid fa-star";
                } else {
                    drawerStar.classList.remove("favorited");
                    if (icon) icon.className = "fa-regular fa-star";
                }
            }
            const miniStar = document.getElementById("mini-btn-favorite");
            if (miniStar) {
                const icon = miniStar.querySelector("i");
                if (isFav) {
                    miniStar.classList.add("favorited");
                    if (icon) icon.className = "fa-solid fa-star";
                } else {
                    miniStar.classList.remove("favorited");
                    if (icon) icon.className = "fa-regular fa-star";
                }
            }
        }

        // If caching detail track, invalidate/update cache
        if (trackDetailsCache[trackId]) {
            trackDetailsCache[trackId].favorite_count = data.favorite_count;
            trackDetailsCache[trackId].user_affinity = data.user_affinity;
        }

        // Refresh Favorites workspace if active
        if (state.activeWorkspace === "workspace-favorites") {
            loadFavoritesWorkspace();
        }

    } catch (err) {
        console.error("Error toggling favorite:", err);
    }
}

// Toggle Dislike status in database and sync UI
async function toggleDislike(trackId) {
    if (!trackId) {
        trackId = state.activeTrackId || (typeof remote_status !== "undefined" && remote_status.track_id);
    }
    if (!trackId) return;
    try {
        const res = await fetch(`/api/interact?id=${trackId}&type=dislike`);
        if (!res.ok) throw new Error("Dislike API failed");
        const data = await res.json();

        const isDis = data.disliked > 0;
        const isFav = data.favorite_count > 0;

        // 1. Sync favorite stars in list items
        const starBtns = document.querySelectorAll(`button.btn-favorite[data-id="${trackId}"]`);
        starBtns.forEach(btn => {
            const icon = btn.querySelector("i");
            if (isFav) {
                btn.classList.add("favorited");
                if (icon) icon.className = "fa-solid fa-star";
            } else {
                btn.classList.remove("favorited");
                if (icon) icon.className = "fa-regular fa-star";
            }
        });

        // 2. Sync all dislike buttons across views
        ["btn-drawer-dislike", "mini-btn-dislike", "fs-btn-dislike", "overlay-btn-dislike"].forEach(id => {
            const btn = document.getElementById(id);
            if (btn) {
                const icon = btn.querySelector("i");
                if (isDis) {
                    btn.classList.add("disliked");
                    btn.style.color = "var(--accent-purple, #a855f7)";
                    if (icon) icon.className = "fa-solid fa-thumbs-down";
                } else {
                    btn.classList.remove("disliked");
                    btn.style.color = "var(--text-mid)";
                    if (icon) icon.className = "fa-regular fa-thumbs-down";
                }
            }
        });

        // 3. Sync all favorite buttons across views
        ["btn-drawer-favorite", "mini-btn-favorite", "fs-btn-favorite", "overlay-btn-favorite"].forEach(id => {
            const btn = document.getElementById(id);
            if (btn) {
                const icon = btn.querySelector("i");
                if (isFav) {
                    btn.classList.add("favorited");
                    btn.style.color = "var(--accent-purple, #a855f7)";
                    if (icon) icon.className = "fa-solid fa-star";
                } else {
                    btn.classList.remove("favorited");
                    btn.style.color = "var(--text-mid)";
                    if (icon) icon.className = "fa-regular fa-star";
                }
            }
        });

        // Update track object in activePlaylist
        const targetPlaylistTrack = state.activePlaylist.find(t => Number(t.id) === Number(trackId));
        if (targetPlaylistTrack) {
            targetPlaylistTrack.disliked = data.disliked;
            targetPlaylistTrack.favorite_count = data.favorite_count;
        }

        if (isDis) {
            state.activePlaylist = state.activePlaylist.filter(t => Number(t.id) !== Number(trackId));
            generateShuffleIndices();
        }

        // Refresh Favorites workspace if active
        if (state.activeWorkspace === "workspace-favorites") {
            loadFavoritesWorkspace();
        }

        // Auto advance to next song if the currently playing song is disliked!
        if (isDis && (Number(state.activeTrackId) === Number(trackId) || (typeof remote_status !== "undefined" && Number(remote_status.track_id) === Number(trackId)))) {
            playNextTrack();
        }
    } catch (err) {
        console.error("Error toggling dislike:", err);
    }
}

// Render Pagination footer
function renderPagination(total, page, limit) {
    const start = total === 0 ? 0 : (page - 1) * limit + 1;
    const end = Math.min(total, page * limit);

    paginationInfo.textContent = `Showing ${start} to ${end} of ${total} tracks`;
    paginationCurrent.textContent = `Page ${page} of ${state.totalPages}`;

    if (paginationFirst) paginationFirst.disabled = page === 1;
    if (paginationPrev) paginationPrev.disabled = page === 1;
    if (paginationNext) paginationNext.disabled = page === state.totalPages;
    if (paginationLast) paginationLast.disabled = page === state.totalPages;
}

// Update Search Clear Button visibility based on searchInput value
function updateSearchClearBtnVisibility() {
    if (searchClearBtn && searchInput) {
        searchClearBtn.style.display = searchInput.value ? "block" : "none";
    }
}

// Start search placeholders rotation
const searchPlaceholders = [
    "e.g., fast strings and key changes",
    "e.g., epic climax strength > 80%",
    "e.g., vocal dreaminess calm",
    "e.g., composer fox capture plan",
    "e.g., time of day feel: Night",
    "e.g., B Major Cinematicness > 70%"
];
let searchPlaceholderIndex = 0;

function startSearchPlaceholderRotation() {
    if (!searchInput) return;
    setInterval(() => {
        searchPlaceholderIndex = (searchPlaceholderIndex + 1) % searchPlaceholders.length;
        searchInput.placeholder = searchPlaceholders[searchPlaceholderIndex];
    }, 5000);
}

// Calculate Narrative Journey Type based on temporal segments
function getJourneyType(track) {
    const se = track.start_energy || 0.5;
    const ee = track.end_energy || 0.5;
    const sc = track.start_calmness || 0.5;
    const ec = track.end_calmness || 0.5;
    const sv = track.start_valence || 0.5;
    const ev = track.end_valence || 0.5;

    if (se < 0.45 && ee > 0.65) return "Calm → Epic Climax";
    if (se > 0.65 && ee < 0.45) return "Energetic → Serene Outro";
    if (sc > 0.6 && ec < 0.4) return "Reflective → Dramatic Build";
    if (sv < 0.45 && ev > 0.6) return "Nostalgic → Hopeful Triumph";
    if (sv > 0.6 && ev < 0.45) return "Hopeful → Tense Conflict";
    if (se < 0.4 && ee < 0.4) return "Atmospheric Ambient Journey";
    if (se > 0.7 && ee > 0.7) return "High-Intensity Action Loop";
    return "Balanced Narrative";
}

// Format summary header info inside Intelligence drawer
function renderIntelligenceSummary(track) {
    const headerEl = document.getElementById("intel-summary-header");
    if (!headerEl) return;

    // Formulate variables
    const cleanKey = track.musical_key || "Unknown Key";
    const cleanMode = track.major_minor || "";
    // Correct key signature duplicate naming
    const keyStr = cleanKey.toLowerCase().includes(cleanMode.toLowerCase())
        ? cleanKey
        : `${cleanKey} ${cleanMode}`.trim();

    const emotionStr = track.emotion_primary ? track.emotion_primary.toUpperCase() : "BALANCED";
    const characterStr = track.audio_character ? track.audio_character.split(' ')[0].replace('/', '') : "MODERATE";
    const peakStr = track.peak_timestamp ? formatDuration(track.peak_timestamp) : "N/A";
    const journey = getJourneyType(track);
    const confidencePct = Math.round((track.overall_analysis_confidence || 0.94) * 100);

    let subVibe = [];
    if (track.dreaminess > 0.6) subVibe.push("Dreamy");
    if (track.cinematicness > 0.6) subVibe.push("Cinematic");
    if (track.epicness > 0.6) subVibe.push("Epic");
    if (subVibe.length === 0) subVibe.push("Atmospheric");

    const cleanEmotionValue = track.emotion_primary || "";
    const cleanCharacterValue = track.audio_character || "";
    const cleanKeyVal = track.musical_key || "";
    const cleanScaleVal = track.major_minor || "";

    headerEl.innerHTML = `
        <div class="identity">
            <span class="interactive-tag tag-clickable" style="cursor: pointer; color: var(--accent-magenta); font-weight: 700; border: 1px solid rgba(236, 72, 153, 0.2); padding: 2px 6px; border-radius: 4px; background: rgba(236, 72, 153, 0.05); font-size: 11px;" onclick="applyLibraryFilter('emotion', '${cleanEmotionValue}')">${emotionStr}</span>
            <span class="interactive-tag tag-clickable" style="cursor: pointer; color: var(--accent-purple); font-weight: 700; border: 1px solid rgba(192, 132, 252, 0.2); padding: 2px 6px; border-radius: 4px; background: rgba(192, 132, 252, 0.05); font-size: 11px;" onclick="applyLibraryFilter('character', '${cleanCharacterValue}')">${characterStr}</span>
            <span style="font-size: 11px; opacity: 0.6;">ORCHESTRAL</span>
        </div>
        <div class="attributes" style="margin-top: 8px; display: flex; flex-wrap: wrap; gap: 6px;">
            <span class="interactive-tag tag-clickable" style="cursor: pointer; color: var(--accent-cyan); font-weight: 700; border: 1px solid rgba(34, 211, 238, 0.2); padding: 2px 6px; border-radius: 4px; background: rgba(34, 211, 238, 0.05); font-size: 11px;" onclick="applyLibraryFilter('key', '${cleanKeyVal}')">${track.musical_key || "Key"}</span>
            <span class="interactive-tag tag-clickable" style="cursor: pointer; color: var(--accent-cyan); font-weight: 700; border: 1px solid rgba(34, 211, 238, 0.2); padding: 2px 6px; border-radius: 4px; background: rgba(34, 211, 238, 0.05); font-size: 11px;" onclick="applyLibraryFilter('scale', '${cleanScaleVal}')">${cleanScaleVal || "Scale"}</span>
            ${subVibe.map(v => `<span class="interactive-tag tag-clickable" style="cursor: pointer; color: var(--accent-emerald); font-weight: 700; border: 1px solid rgba(16, 185, 129, 0.2); padding: 2px 6px; border-radius: 4px; background: rgba(16, 185, 129, 0.05); font-size: 11px;" onclick="applyLibraryFilter('${v.toLowerCase()}', 'high')">${v}</span>`).join('')}
        </div>
        <div class="meta-stats" style="margin-top: 8px;"><i class="fa-solid fa-route"></i> Journey: <strong class="text-accent">${journey}</strong></div>
        <div class="meta-stats"><i class="fa-solid fa-fire"></i> Climax: ${peakStr} (Strength: ${((track.climax_strength || 0) * 100).toFixed(0)}%)</div>
        
        <div class="vibe-stat-block" style="margin-top: 10px; margin-bottom: 0;">
            <div class="stat-header" style="font-size: 11px; margin-bottom: 4px;">
                <span class="label"><i class="fa-solid fa-square-poll-vertical"></i> Analysis Quality</span>
                <span class="value" style="color: var(--accent-emerald); font-weight: 800;">${confidencePct}%</span>
            </div>
            <div class="progress-bar-bg" style="height: 4px; background: rgba(255,255,255,0.03);">
                <div class="progress-bar-fill fill-emerald" style="width: ${confidencePct}%; height: 100%;"></div>
            </div>
        </div>
    `;

    // Also sync to fullscreen specs
    const fsVibe = document.getElementById("fs-val-summary-vibe");
    if (fsVibe) fsVibe.textContent = `${emotionStr} ${characterStr} ORCHESTRAL`;
    const fsKey = document.getElementById("fs-val-key");
    if (fsKey) fsKey.textContent = keyStr;
    const fsJourney = document.getElementById("fs-val-journey");
    if (fsJourney) fsJourney.textContent = journey;
}

// Draw dynamic SVG Radar Fingerprint Chart
function renderRadarChart(track, containerId) {
    const container = document.getElementById(containerId);
    if (!container) return;

    const w = 240;
    const h = 210;
    const centerX = 120;
    const centerY = 105;
    const rMax = 58;

    // Dimensions mapping
    const metrics = [
        { label: "Dreamy", val: track.dreaminess || 0.5 },
        { label: "Epic", val: track.epicness || 0.5 },
        { label: "Energy", val: track.energy || 0.5 },
        { label: "Calm", val: track.calmness || 0.5 },
        { label: "Cinema", val: track.cinematicness || 0.5 },
        { label: "Focus", val: track.focus_score || 0.5 }
    ];

    let gridLines = "";
    [0.33, 0.66, 1.0].forEach(level => {
        const rad = rMax * level;
        const pts = [];
        for (let i = 0; i < 6; i++) {
            const angle = (i * Math.PI) / 3 - Math.PI / 2;
            const x = centerX + rad * Math.cos(angle);
            const y = centerY + rad * Math.sin(angle);
            pts.push(`${x.toFixed(1)},${y.toFixed(1)}`);
        }
        gridLines += `<polygon points="${pts.join(" ")}" fill="none" stroke="rgba(255,255,255,0.06)" stroke-width="1" />`;
    });

    let webLines = "";
    for (let i = 0; i < 6; i++) {
        const angle = (i * Math.PI) / 3 - Math.PI / 2;
        const x = centerX + rMax * Math.cos(angle);
        const y = centerY + rMax * Math.sin(angle);
        webLines += `<line x1="${centerX}" y1="${centerY}" x2="${x.toFixed(1)}" y2="${y.toFixed(1)}" stroke="rgba(255,255,255,0.06)" stroke-width="1" />`;
    }

    const polyPts = [];
    metrics.forEach((m, i) => {
        const angle = (i * Math.PI) / 3 - Math.PI / 2;
        const rad = rMax * Math.max(0.1, Math.min(1.0, m.val));
        const x = centerX + rad * Math.cos(angle);
        const y = centerY + rad * Math.sin(angle);
        polyPts.push(`${x.toFixed(1)},${y.toFixed(1)}`);
    });

    let labels = "";
    metrics.forEach((m, i) => {
        const angle = (i * Math.PI) / 3 - Math.PI / 2;
        const offset = 18;
        const x = centerX + (rMax + offset) * Math.cos(angle);
        const y = centerY + (rMax + offset) * Math.sin(angle);

        let anchor = "middle";
        const cosA = Math.cos(angle);
        if (cosA > 0.3) anchor = "start";
        else if (cosA < -0.3) anchor = "end";

        let dy = 4;
        const sinA = Math.sin(angle);
        if (sinA < -0.8) dy = -4;
        if (sinA > 0.8) dy = 10;

        labels += `<text x="${x.toFixed(1)}" y="${(y + dy).toFixed(1)}" fill="var(--accent-cyan, #22d3ee)" font-size="10" font-weight="700" letter-spacing="0.04em" text-anchor="${anchor}">${m.label}</text>`;
    });

    container.innerHTML = `
        <svg viewBox="0 0 ${w} ${h}" class="radar-svg" style="width:100%; height:100%; overflow:visible;">
            <defs>
                <radialGradient id="radar-glow-${containerId}" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" stop-color="#22d3ee" stop-opacity="0.35"/>
                    <stop offset="100%" stop-color="#c084fc" stop-opacity="0.05"/>
                </radialGradient>
            </defs>
            ${gridLines}
            ${webLines}
            <polygon points="${polyPts.join(" ")}" fill="url(#radar-glow-${containerId})" stroke="#22d3ee" stroke-width="2.2" style="filter: drop-shadow(0 0 8px rgba(34, 211, 238, 0.5));" />
            ${labels}
        </svg>
    `;
}

// Draw dynamic SVG Emotion Arc Bezier Waves
function renderEmotionArcChart(track, containerId) {
    const container = document.getElementById(containerId);
    if (!container) return;

    const w = 320;
    const h = 80;
    const padding = 15;

    // Emotion components arrays
    const dataList = [
        { label: "Energy", stroke: "var(--accent-purple)", values: [track.start_energy || 0.4, track.mid_energy || 0.6, track.end_energy || 0.5] },
        { label: "Calm", stroke: "var(--accent-cyan)", values: [track.start_calmness || 0.5, track.mid_calmness || 0.3, track.end_calmness || 0.6] },
        { label: "Valence", stroke: "#34d399", values: [track.start_valence || 0.5, track.mid_valence || 0.5, track.end_valence || 0.5] },
        { label: "Arousal", stroke: "#fb7185", values: [track.start_arousal || 0.4, track.mid_arousal || 0.7, track.end_arousal || 0.3] }
    ];

    function makeBezierPath(arr) {
        const x0 = padding;
        const y0 = h - padding - arr[0] * (h - 2 * padding);
        const x1 = w / 2;
        const y1 = h - padding - arr[1] * (h - 2 * padding);
        const x2 = w - padding;
        const y2 = h - padding - arr[2] * (h - 2 * padding);

        // Quad curve string
        return `M ${x0} ${y0} Q ${x1} ${y1} ${x2} ${y2}`;
    }

    let pathHtml = "";
    dataList.forEach(d => {
        const pathStr = makeBezierPath(d.values);
        pathHtml += `<path d="${pathStr}" fill="none" stroke="${d.stroke}" stroke-width="2" style="filter: drop-shadow(0 0 4px ${d.stroke}); opacity: 0.85;" />`;
    });

    // Legend indicators
    let legend = `<div style="display:flex; justify-content:space-around; margin-top:8px;">`;
    dataList.forEach(d => {
        legend += `<span style="font-size:10px; font-weight:700; color:${d.stroke}; display:flex; align-items:center; gap:4px;"><span style="width:8px; height:8px; background:${d.stroke}; border-radius:50%;"></span>${d.label}</span>`;
    });
    legend += `</div>`;

    container.innerHTML = `
        <svg viewBox="0 0 ${w} ${h}" class="emotion-svg" style="background:rgba(0,0,0,0.15); border-radius:8px;">
            <line x1="${padding}" y1="${h - padding}" x2="${w - padding}" y2="${h - padding}" stroke="rgba(255,255,255,0.05)" stroke-dasharray="2" />
            <line x1="${padding}" y1="${h / 2}" x2="${w - padding}" y2="${h / 2}" stroke="rgba(255,255,255,0.02)" stroke-dasharray="2" />
            ${pathHtml}
        </svg>
        ${legend}
    `;
}

// Render click-to-play timeline block sections
function renderSectionTimeline(track) {
    const container = document.getElementById("timeline-container-v2");
    if (!container) return;

    const sections = track.section_summary || (track.section_summary_json ? JSON.parse(track.section_summary_json) : null);

    if (!sections || sections.length === 0) {
        container.innerHTML = `<span class="no-themes">Acoustic sections not analyzed yet.</span>`;
        return;
    }

    try {
        const totalDuration = track.duration || 1;
        let html = "";

        sections.forEach((sec, idx) => {
            const start = sec.start !== undefined ? sec.start : (sec.start_time || 0);
            const end = sec.end !== undefined ? sec.end : (sec.end_time || 0);
            const widthPct = ((end - start) / totalDuration) * 100;
            const label = sec.label || `Sec ${idx + 1}`;

            html += `
                <div class="timeline-sec" id="timeline-sec-${idx}" style="width: ${widthPct}%; cursor:pointer;" onclick="seekAudioTo(${start})" title="${label}: ${formatDuration(start)} - ${formatDuration(end)}">
                    <span class="sec-label">${escapeHtml(label)}</span>
                </div>
            `;
        });

        container.innerHTML = html;
    } catch (e) {
        container.innerHTML = `<span class="no-themes">Error parsing timeline json metrics.</span>`;
    }
}

function seekAudioTo(sec) {
    state.localPlayTimeSec = sec;
    state.lastProgressUpdateTime = performance.now();
    state.lastSeekTimestamp = Date.now();
    audioTimeCurrent.textContent = formatDuration(sec);
    if (state.duration > 0) {
        setAudioSliderProgress((sec / state.duration) * 100);
    }
    fetch("/api/player/seek", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ time: sec }) });
    if (!state.isPlaying) {
        playAudio();
    }
}

// Render dynamic instrument density tracks
function renderInstrumentHeatmap(track) {
    const container = document.getElementById("heatmap-container");
    if (!container) return;

    try {
        const getOpacity = (v) => {
            if (v <= 0.01) return 0.0;
            return 0.30 + 0.70 * Math.min(1.0, Math.max(0.0, v));
        };

        let presenceTimeline = track.instrument_presence_timeline;
        if (typeof presenceTimeline === "string") {
            try {
                presenceTimeline = JSON.parse(presenceTimeline);
            } catch (err) {
                presenceTimeline = null;
            }
        }

        const instrumentConfigs = [
            { key: "strings", label: "Strings", startColor: "#881337", endColor: "#ec4899", sources: ["strings", "guitar", "bass"] }, // Strings / Chordophones
            { key: "keyboards", label: "Keyboards", startColor: "#083344", endColor: "#06b6d4", sources: ["piano", "synth"] },   // Keyboards / Electronics
            { key: "choir", label: "Choir", startColor: "#064e3b", endColor: "#10b981", sources: ["choir"] },                 // Choir / Vocals
            { key: "drums", label: "Percussion", startColor: "#7f1d1d", endColor: "#f43f5e", sources: ["drums"] },            // Percussion / Drums
            { key: "winds", label: "Woodwinds", startColor: "#1e3a8a", endColor: "#3b82f6", sources: ["winds"] },             // Woodwinds
            { key: "brass", label: "Brass", startColor: "#451a03", endColor: "#f59e0b", sources: ["brass"] }                  // Brass
        ];

        // Color interpolation helper
        const interpolateColor = (color1, color2, factor) => {
            const parseHex = (c) => {
                const match = c.replace("#", "").match(/.{2}/g);
                return match ? match.map(x => parseInt(x, 16)) : [0, 0, 0];
            };
            const [r1, g1, b1] = parseHex(color1);
            const [r2, g2, b2] = parseHex(color2);
            const r = Math.round(r1 + factor * (r2 - r1));
            const g = Math.round(g1 + factor * (g2 - g1));
            const b = Math.round(b1 + factor * (b2 - b1));
            return `rgb(${r}, ${g}, ${b})`;
        };

        let html = "";
        const totalBlocks = 60;
        const duration = track.duration || 180;

        // Calculate overall volume (sum of all instrument intensities) for each of the 60 blocks
        let blockVolumes = [];
        if (presenceTimeline && presenceTimeline.length > 0) {
            const chunkSize = presenceTimeline.length / totalBlocks;
            for (let b = 0; b < totalBlocks; b++) {
                const startIdx = Math.floor(b * chunkSize);
                const endIdx = Math.floor((b + 1) * chunkSize);

                let sum = 0, count = 0;
                for (let idx = startIdx; idx < endIdx; idx++) {
                    const entry = presenceTimeline[idx];
                    if (entry) {
                        if (entry.volume !== undefined) {
                            sum += parseFloat(entry.volume);
                        } else {
                            let frameSum = 0;
                            Object.keys(entry).forEach(key => {
                                if (key !== 't' && key !== 'chord') {
                                    frameSum += parseFloat(entry[key] || 0);
                                }
                            });
                            sum += frameSum;
                        }
                        count++;
                    }
                }
                blockVolumes.push(count > 0 ? sum / count : 0.0);
            }

            const maxVolume = Math.max(...blockVolumes);
            if (maxVolume > 0) {
                blockVolumes = blockVolumes.map(v => 0.20 + 0.80 * (v / maxVolume));
            } else {
                blockVolumes = blockVolumes.map(() => 1.0);
            }
        } else {
            blockVolumes = Array(totalBlocks).fill(1.0);
        }

        instrumentConfigs.forEach(config => {
            let presencePct = 0;

            const isMLInstrument = ["strings", "keyboards", "winds", "brass"].includes(config.key);
            const noiseThreshold = isMLInstrument ? 0.00 : 0.02;
            const scaleFactor = isMLInstrument ? 4.0 : 1.0;

            // Compute composite score from sources
            let compositeScore = 0.0;
            let hasValidSource = false;
            config.sources.forEach(src => {
                let scoreVal = track[src + "_score"];
                if (scoreVal !== undefined && scoreVal !== null) {
                    compositeScore = Math.max(compositeScore, parseFloat(scoreVal));
                    hasValidSource = true;
                }
            });

            if (hasValidSource) {
                if (compositeScore < noiseThreshold) {
                    presencePct = 0;
                } else {
                    presencePct = Math.min(100, Math.round(compositeScore * 100 * scaleFactor));
                }
            } else if (presenceTimeline && presenceTimeline.length > 0) {
                let sum = 0, count = 0;
                presenceTimeline.forEach(entry => {
                    let maxVal = 0.0;
                    let found = false;
                    config.sources.forEach(src => {
                        let name = src === "keyboards" ? "piano" : src; // map keyboards to piano/synth in timeline
                        let t_val = entry[name];
                        if (t_val !== undefined) {
                            maxVal = Math.max(maxVal, parseFloat(t_val));
                            found = true;
                        }
                    });
                    if (found) {
                        if (maxVal >= noiseThreshold) {
                            sum += maxVal;
                            count++;
                        }
                    }
                });
                if (count > 0) {
                    presencePct = Math.min(100, Math.round((sum / count) * 100 * scaleFactor));
                }
            }

            // Calculate chunk-averaged values for the 60 blocks first to find true peak/avg of the rendered blocks
            let blockVals = [];
            if (presenceTimeline && presenceTimeline.length > 0) {
                const chunkSize = presenceTimeline.length / totalBlocks;
                if (chunkSize >= 1.0) {
                    for (let b = 0; b < totalBlocks; b++) {
                        const startIdx = Math.floor(b * chunkSize);
                        const endIdx = Math.floor((b + 1) * chunkSize);

                        let sum = 0, count = 0;
                        for (let idx = startIdx; idx < endIdx; idx++) {
                            if (presenceTimeline[idx]) {
                                let maxVal = 0.0;
                                config.sources.forEach(src => {
                                    let t_val = presenceTimeline[idx][src];
                                    if (t_val !== undefined) {
                                        maxVal = Math.max(maxVal, parseFloat(t_val));
                                    }
                                });
                                sum += maxVal < noiseThreshold ? 0.0 : maxVal;
                                count++;
                            }
                        }
                        blockVals.push(count > 0 ? sum / count : 0.0);
                    }
                } else {
                    // Fallback for smaller/lower resolution timelines to prevent dotted gaps:
                    // Use nearest-neighbor lookup to fill cells smoothly
                    for (let b = 0; b < totalBlocks; b++) {
                        const idx = Math.min(
                            presenceTimeline.length - 1,
                            Math.floor((b / totalBlocks) * presenceTimeline.length)
                        );
                        let maxVal = 0.0;
                        if (presenceTimeline[idx]) {
                            config.sources.forEach(src => {
                                let t_val = presenceTimeline[idx][src];
                                if (t_val !== undefined) {
                                    maxVal = Math.max(maxVal, parseFloat(t_val));
                                }
                            });
                        }
                        if (maxVal < noiseThreshold) maxVal = 0.0;
                        blockVals.push(maxVal);
                    }
                }
            }

            let timelineAvg = 0.5;
            let timelineMax = 1.0;
            if (blockVals.length > 0) {
                const sum = blockVals.reduce((a, b) => a + b, 0);
                timelineAvg = sum / blockVals.length;
                timelineMax = Math.max(...blockVals);
            }

            // Render heatmap cells
            let cells = "";
            if (presenceTimeline && presenceTimeline.length > 0) {
                for (let b = 0; b < totalBlocks; b++) {
                    const rawVal = blockVals[b];
                    const timePoint = (b + 0.5) * (duration / totalBlocks);

                    // Normalize relative to the track's peak intensity if it is active enough
                    let val = 0.0;
                    if (rawVal >= noiseThreshold && timelineMax > noiseThreshold) {
                        val = rawVal / timelineMax;
                    } else {
                        val = 0.0;
                    }

                    // Interpolate cell color based on intensity value (val) from dark (startColor) to bright (endColor)
                    const cellColor = interpolateColor(config.startColor, config.endColor, val);

                    // Smooth, continuous opacity mapping based on normalized intensity value
                    const opacity = getOpacity(val).toFixed(3);
                    cells += `<div class="heatmap-cell" data-block-index="${b}" style="background:${cellColor}; opacity:${opacity}; cursor:pointer;" onclick="seekAudioTo(${timePoint})" title="${config.label} at ${formatDuration(timePoint)} (intensity: ${Math.round(val * 100)}%, volume: ${Math.round(blockVolumes[b] * 100)}%)"></div>`;
                }
            } else {
                for (let b = 0; b < totalBlocks; b++) {
                    const timePoint = (b + 0.5) * (duration / totalBlocks);
                    // Create a beautiful, simulated wave pattern for fallback states
                    const simulatedVal = presencePct > 0 ? (0.2 + 0.7 * Math.abs(Math.sin((b / 10) + (config.label.length)))) : 0.0;
                    const opacity = getOpacity(simulatedVal).toFixed(3);
                    const cellColor = interpolateColor(config.startColor, config.endColor, simulatedVal);
                    cells += `<div class="heatmap-cell" data-block-index="${b}" style="background:${cellColor}; opacity:${opacity}; cursor:pointer;" onclick="seekAudioTo(${timePoint})"></div>`;
                }
            }

            const rowOpacity = presencePct === 0 ? 0.35 : 1.0;
            const rowGrayscale = presencePct === 0 ? "grayscale(70%)" : "none";
            const labelWeight = Math.max(300, 300 + Math.round(presencePct * 6));
            const gridHeight = Math.max(10, 10 + Math.round(presencePct * 0.14));
            const fontStyle = `font-weight: ${labelWeight};`;
            const textShadow = presencePct > 0 ? `text-shadow: 0 0 ${Math.max(2, Math.round(presencePct / 8))}px ${config.endColor};` : "";

            html += `
                <div class="heatmap-row" style="margin-bottom: 8px; opacity: ${rowOpacity}; filter: ${rowGrayscale}; transition: all 0.3s ease;">
                    <div class="heatmap-label" style="display:flex; justify-content:space-between; align-items:center; width:100%; font-size:10px; margin-bottom: 2px;">
                        <span style="${fontStyle} ${textShadow} color:${config.endColor}; cursor:pointer;" onclick="applyLibraryFilter('${config.key}', 'high')">${config.label}</span>
                        <span style="font-size:9px; color:${presencePct > 0 ? 'rgba(255,255,255,0.7)' : 'rgba(255,255,255,0.25)'}; font-weight:${labelWeight}">Presence: ${presencePct}%</span>
                    </div>
                    <div class="heatmap-grid" style="display:flex; height:${gridHeight}px;">
                        ${cells}
                    </div>
                </div>
            `;
        });

        // Add a dedicated Chord Progression Row at the top of the heatmap if chord data exists in presenceTimeline
        if (presenceTimeline && presenceTimeline.length > 0) {
            const parsedTimeline = typeof presenceTimeline === "string" ? JSON.parse(presenceTimeline) : presenceTimeline;
            if (parsedTimeline && parsedTimeline.length > 0) {
                const hasChords = parsedTimeline.some(entry => entry.chord);
                if (hasChords) {
                    let chordCells = "";
                    const chunkSize = parsedTimeline.length / totalBlocks;

                    for (let b = 0; b < totalBlocks; b++) {
                        const timePoint = (b + 0.5) * (duration / totalBlocks);
                        const startIdx = Math.floor(b * chunkSize);
                        const endIdx = Math.floor((b + 1) * chunkSize);

                        // Get the most common chord in this block chunk
                        let chordCounts = {};
                        for (let idx = startIdx; idx < endIdx; idx++) {
                            if (parsedTimeline[idx] && parsedTimeline[idx].chord) {
                                const ch = parsedTimeline[idx].chord;
                                if (ch !== "Unknown" && ch !== "None") {
                                    chordCounts[ch] = (chordCounts[ch] || 0) + 1;
                                }
                            }
                        }

                        let bestChord = "";
                        let maxCount = 0;
                        for (const ch in chordCounts) {
                            if (chordCounts[ch] > maxCount) {
                                maxCount = chordCounts[ch];
                                bestChord = ch;
                            }
                        }

                        // Style for chord cell
                        let cellContent = bestChord || "";
                        let cellBg = cellContent ? "rgba(236, 72, 153, 0.12)" : "rgba(255, 255, 255, 0.02)";
                        let cellBorder = cellContent ? "1px solid rgba(236, 72, 153, 0.25)" : "1px solid rgba(255, 255, 255, 0.05)";
                        let fontColor = "var(--accent-magenta)";

                        chordCells += `
                            <div class="heatmap-cell chord-heatmap-cell" 
                                 data-block-index="${b}" 
                                 style="background:${cellBg}; border:${cellBorder}; color:${fontColor}; font-size: 7px; font-weight: 900; display: flex; align-items: center; justify-content: center; cursor: pointer; flex: 1; margin-right: 1px; transition: all 0.2s ease; line-height: 1; text-align: center;" 
                                 onclick="seekAudioTo(${timePoint})" 
                                 title="Chord: ${cellContent || 'None'} at ${formatDuration(timePoint)}">
                                 ${cellContent}
                            </div>
                        `;
                    }

                    html = `
                        <div class="heatmap-row chord-row" style="margin-bottom: 14px; transition: all 0.3s ease; position: relative; z-index: 10;">
                            <div class="heatmap-label" style="display:flex; justify-content:space-between; align-items:center; width:100%; font-size:10px; margin-bottom: 2px;">
                                <span style="font-weight: 700; color: var(--accent-magenta); text-shadow: 0 0 4px rgba(236, 72, 153, 0.4);"><i class="fa-solid fa-guitar"></i> Chord Progression</span>
                                <span style="font-size:9px; color:rgba(255,255,255,0.7); font-weight:700">Timeline Mode</span>
                            </div>
                            <div class="heatmap-grid" style="display:flex; height: 18px; border-radius: 4px; background: rgba(0, 0, 0, 0.25); padding: 1px; position: relative;">
                                ${chordCells}
                            </div>
                        </div>
                    ` + html;
                }
            }
        }

        container.innerHTML = html;

        // Render Extended Heatmap in Centered Dialog (40 Jamendo Instruments)
        const toggleBtn = document.getElementById("btn-toggle-extended-heatmap");
        const extModal = document.getElementById("extended-analysis-modal");
        const extContainer = document.getElementById("modal-extended-heatmap-container");
        const closeModalBtn = document.getElementById("btn-close-extended-modal");

        let rawTimeline = track.raw_instrument_timeline;
        if (typeof rawTimeline === "string") {
            try {
                rawTimeline = JSON.parse(rawTimeline);
            } catch (err) {
                rawTimeline = null;
            }
        }

        if (toggleBtn && extModal && extContainer) {
            if (rawTimeline && rawTimeline.length > 0) {
                toggleBtn.parentElement.style.display = "flex";
                toggleBtn.innerHTML = `<i class="fa-solid fa-chart-pie"></i>`;
                toggleBtn.title = "Open 40-Instrument Acoustic Analysis";

                // Replace clone to wipe previous click event listeners
                const newToggleBtn = toggleBtn.cloneNode(true);
                toggleBtn.parentNode.replaceChild(newToggleBtn, toggleBtn);

                newToggleBtn.addEventListener("click", () => {
                    extModal.style.display = "flex";
                    document.getElementById("modal-track-title").textContent = track.title + " - " + track.artist;
                });

                if (closeModalBtn) {
                    const newCloseBtn = closeModalBtn.cloneNode(true);
                    closeModalBtn.parentNode.replaceChild(newCloseBtn, closeModalBtn);
                    newCloseBtn.addEventListener("click", () => {
                        extModal.style.display = "none";
                    });
                }

                // Render the extended heatmap rows (larger for modal)
                const allInstruments = [
                    "accordion", "acousticbassguitar", "acousticguitar", "bass", "beat", "bell", "bongo", "brass",
                    "cello", "clarinet", "classicalguitar", "computer", "doublebass", "drummachine", "drums",
                    "electricguitar", "electricpiano", "flute", "guitar", "harmonica", "harp", "horn", "keyboard",
                    "oboe", "orchestra", "organ", "pad", "percussion", "piano", "pipeorgan", "rhodes", "sampler",
                    "saxophone", "strings", "synthesizer", "trombone", "trumpet", "viola", "violin", "voice"
                ];

                let activeInsts = [];
                allInstruments.forEach(inst => {
                    let maxVal = 0.0;
                    rawTimeline.forEach(frame => {
                        if (frame.profile && frame.profile[inst] !== undefined) {
                            maxVal = Math.max(maxVal, parseFloat(frame.profile[inst]));
                        }
                    });
                    if (maxVal > 0.015) {
                        activeInsts.push({ name: inst, maxVal: maxVal });
                    }
                });

                activeInsts.sort((a, b) => b.maxVal - a.maxVal);

                let extHtml = "";
                activeInsts.forEach(instObj => {
                    const inst = instObj.name;

                    let color = "#a855f7"; // default purple
                    if (inst.includes("guitar") || inst === "bass" || inst === "doublebass" || inst === "acousticbassguitar") color = "#10b981"; // emerald
                    if (inst.includes("drum") || inst === "beat" || inst === "percussion" || inst === "bongo" || inst === "drummachine" || inst === "sampler") color = "#f43f5e"; // rose
                    if (inst === "violin" || inst === "cello" || inst === "viola" || inst === "strings" || inst === "orchestra" || inst === "harp") color = "#ec4899"; // pink
                    if (inst === "piano" || inst === "keyboard" || inst === "synthesizer" || inst === "organ" || inst === "electricpiano" || inst === "rhodes" || inst === "pad") color = "#06b6d4"; // cyan
                    if (inst === "flute" || inst === "clarinet" || inst === "oboe" || inst === "saxophone" || inst === "brass" || inst === "trumpet" || inst === "trombone" || inst === "horn") color = "#f59e0b"; // yellow

                    let cells = "";
                    const chunkSize = rawTimeline.length / totalBlocks;

                    for (let b = 0; b < totalBlocks; b++) {
                        const timePoint = (b + 0.5) * (duration / totalBlocks);
                        const startIdx = Math.floor(b * chunkSize);
                        const endIdx = Math.floor((b + 1) * chunkSize);

                        let sum = 0, count = 0;
                        for (let idx = startIdx; idx < endIdx; idx++) {
                            if (rawTimeline[idx] && rawTimeline[idx].profile && rawTimeline[idx].profile[inst] !== undefined) {
                                sum += parseFloat(rawTimeline[idx].profile[inst]);
                                count++;
                            }
                        }

                        const avgVal = count > 0 ? (sum / count) : 0.0;
                        const val = avgVal / instObj.maxVal;
                        const cellColor = interpolateColor("#111111", color, val);
                        const opacity = getOpacity(val).toFixed(3);

                        cells += `<div class="heatmap-cell" data-block-index="${b}" style="background:${cellColor}; opacity:${opacity}; cursor:pointer;" onclick="seekAudioTo(${timePoint})" title="${inst} at ${formatDuration(timePoint)} (intensity: ${Math.round(val * 100)}%, volume: ${Math.round(blockVolumes[b] * 100)}%)"></div>`;
                    }

                    let label = inst.charAt(0).toUpperCase() + inst.slice(1);
                    if (label === "Acousticguitar") label = "Acoustic Guitar";
                    if (label === "Electricguitar") label = "Electric Guitar";
                    if (label === "Classicalguitar") label = "Classical Guitar";
                    if (label === "Doublebass") label = "Double Bass";
                    if (label === "Acousticbassguitar") label = "Acoustic Bass Guitar";
                    if (label === "Drummachine") label = "Drum Machine";
                    if (label === "Electricpiano") label = "Electric Piano";
                    if (label === "Pipeorgan") label = "Pipe Organ";

                    extHtml += `
                        <div class="heatmap-row" style="margin-bottom: 8px; transition: all 0.3s ease;">
                            <div class="heatmap-label" style="display:flex; justify-content:space-between; align-items:center; width:100%; font-size:10px; margin-bottom: 2px;">
                                <span style="font-weight: 600; color:${color};">${label}</span>
                                <span style="font-size:9px; color:rgba(255,255,255,0.5);">Max: ${Math.round(instObj.maxVal * 100)}%</span>
                            </div>
                            <div class="heatmap-grid" style="display:flex; height: 14px;">
                                ${cells}
                            </div>
                        </div>
                    `;
                });

                extContainer.innerHTML = extHtml;
            } else {
                toggleBtn.parentElement.style.display = "none";
            }
        }
    } catch (e) {
        console.error("Heatmap rendering error:", e);
        container.innerHTML = `<span class="no-themes">Instrument timeline data unavailable.</span>`;
    }
}


// Render explainable similarity bar layout recommendations
function renderRecommendations(similarTracks) {
    const container = document.getElementById("recommendations-container-v2");
    if (!container) return;

    if (!similarTracks || similarTracks.length === 0) {
        container.innerHTML = `<span class="no-themes">No neural recommendation links found.</span>`;
        return;
    }

    let html = "";
    similarTracks.slice(0, 3).forEach(st => {
        // Calculate sub-similarity metrics
        const emoScore = Math.round((st.similarity_emotion || (st.similarity * 0.95)) * 100);
        const motifScore = Math.round((st.similarity_motif || (st.similarity * 0.9)) * 100);
        const instScore = Math.round((st.similarity_instruments || (st.similarity * 0.85)) * 100);

        html += `
            <div class="recommendation-item" style="border: 1px solid rgba(255,255,255,0.02); background:rgba(0,0,0,0.1); border-radius:8px; padding:10px; margin-bottom:8px;">
                <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:8px;">
                    <div>
                        <div style="font-size:12.5px; font-weight:700; color:var(--text-high); cursor:pointer;" onclick="playImmediate('${st.id}')">${escapeHtml(st.title)}</div>
                        <div style="font-size:10.5px; color:var(--text-low);">${renderArtistLinks(st.artist, false)}</div>
                    </div>
                    <span style="font-size:11px; font-weight:800; color:var(--accent-purple); padding:3px 6px; background:rgba(192,132,252,0.1); border-radius:4px;">${Math.round(st.similarity * 100)}% Match</span>
                </div>
                
                <div style="display:flex; flex-direction:column; gap:4px;">
                    <div style="display:flex; justify-content:space-between; font-size:9.5px; font-weight:600; color:var(--text-low);">
                        <span>Emotion Sim:</span> <span>${emoScore}%</span>
                    </div>
                    <div class="progress-bar-bg" style="height:3px; background:rgba(255,255,255,0.02);"><div class="progress-bar-fill fill-purple" style="width:${emoScore}%; height:100%;"></div></div>
                    
                    <div style="display:flex; justify-content:space-between; font-size:9.5px; font-weight:600; color:var(--text-low);">
                        <span>Motif Sim:</span> <span>${motifScore}%</span>
                    </div>
                    <div class="progress-bar-bg" style="height:3px; background:rgba(255,255,255,0.02);"><div class="progress-bar-fill fill-cyan" style="width:${motifScore}%; height:100%;"></div></div>

                    <div style="display:flex; justify-content:space-between; font-size:9.5px; font-weight:600; color:var(--text-low);">
                        <span>Instrumentation Overlap:</span> <span>${instScore}%</span>
                    </div>
                    <div class="progress-bar-bg" style="height:3px; background:rgba(255,255,255,0.02);"><div class="progress-bar-fill fill-emerald" style="width:${instScore}%; height:100%;"></div></div>
                </div>
            </div>
        `;
    });

    container.innerHTML = html;
}

// Render Motif tree Variations specific to selected track
function renderThemeVariations(variations) {
    const container = document.getElementById("theme-variations-list");
    if (!container) return;

    if (!variations || variations.length === 0) {
        container.innerHTML = `<span class="no-themes">This track does not belong to a detected theme cluster.</span>`;
        return;
    }

    let html = `<div class="theme-family-tree">`;
    variations.forEach((v, i) => {
        const isCurrent = v.id === state.activeTrackId ? "node-main" : "";
        const role = i === 0 ? "Main Theme" : `Variant #${i}`;
        const simBadge = i === 0 ? "" : `<span class="theme-node-badge badge-similarity">Sim: ${Math.round(v.theme_similarity * 100)}%</span>`;

        html += `
            <div class="theme-node ${isCurrent}">
                <div class="theme-node-info" style="cursor:pointer;" onclick="playImmediate('${v.id}')">
                    <span class="theme-node-title">${escapeHtml(v.title)}</span>
                    <span class="theme-node-meta">${escapeHtml(v.artist)}</span>
                </div>
                <div class="theme-node-badges" style="margin-right:12px;">
                    <span class="theme-node-badge badge-importance">Imp: ${(v.theme_importance || 1.0).toFixed(1)}</span>
                    ${simBadge}
                </div>
                <div class="theme-node-actions">
                    <button class="row-play-btn" onclick="playImmediate('${v.id}')" style="padding:4px 8px; font-size:10px;"><i class="fa-solid fa-play"></i></button>
                    <button class="row-play-btn" onclick="event.stopPropagation(); addToQueue('${v.id}')" style="padding:4px 8px; font-size:10px;" title="Add to Playback Queue"><i class="fa-solid fa-plus"></i></button>
                </div>
            </div>
        `;
    });
    html += `</div>`;

    container.innerHTML = html;
}

// Generate YAMNet explainable auto-suggest preset recommendations
function renderDSPRecommendations(track) {
    const container = document.getElementById("dsp-recommended-content");
    if (!container) return;

    let recs = [];
    let score = 0;

    if (track.cinematicness > 0.6) {
        recs.push(`<li><i class="fa-solid fa-check text-accent" style="color:var(--accent-cyan); margin-right:6px;"></i><strong>Cinematic Expansion</strong> (boosts stereo width to 150% and EQ air)</li>`);
        score += 0.3;
    }
    if (track.dreaminess > 0.6) {
        recs.push(`<li><i class="fa-solid fa-check text-accent" style="color:var(--accent-cyan); margin-right:6px;"></i><strong>Reverb Space Ambience</strong> (activates dynamic decay and low-frequency depth)</li>`);
        score += 0.25;
    }
    if (track.vocal_density > 0.3) {
        recs.push(`<li><i class="fa-solid fa-check text-accent" style="color:var(--accent-cyan); margin-right:6px;"></i><strong>Vocal Presence</strong> (activates EQ peaking centered at 2.5kHz)</li>`);
        score += 0.2;
    }
    if (track.epicness > 0.6) {
        recs.push(`<li><i class="fa-solid fa-check text-accent" style="color:var(--accent-cyan); margin-right:6px;"></i><strong>Bass Enhancement</strong> (enables low shelf boost at 100Hz and limiting)</li>`);
        score += 0.25;
    }

    if (recs.length === 0) {
        recs.push(`<li><i class="fa-solid fa-check text-accent" style="color:var(--accent-cyan); margin-right:6px;"></i><strong>Default Balanced flat preset</strong></li>`);
        score = 0.9;
    }

    const confidenceVal = Math.min(98, Math.round((0.7 + score * 0.3) * 100));

    container.innerHTML = `
        <div style="font-size: 13px; font-weight:700; color:var(--text-high); margin-bottom:8px;">Suggested Profile: <strong style="color:var(--accent-purple);">Dynamic Enhancements</strong></div>
        <ul style="padding-left:0; margin-bottom:14px; list-style:none; display:flex; flex-direction:column; gap:6px; font-size:12.5px; color:var(--text-mid);">
            ${recs.join("")}
        </ul>
        <div style="font-size: 11.5px; font-weight:700; color:var(--text-low); margin-bottom:10px;">
            AI Confidence Score: <span class="text-accent" style="color:var(--accent-emerald);">${confidenceVal}%</span>
        </div>
        <button class="action-btn-sm" id="dsp-btn-apply-rec" style="background: rgba(34, 211, 238, 0.15); border-color: rgba(34, 211, 238, 0.3); color: var(--accent-cyan); padding: 8px 14px;">Apply Suggested Chain</button>
    `;

    // Bind Apply click listener
    const applyBtn = document.getElementById("dsp-btn-apply-rec");
    if (applyBtn) {
        applyBtn.addEventListener("click", () => {
            // Preset configuration in proper dB values
            if (track.cinematicness > 0.6) {
                document.getElementById("dsp-cb-stereo").checked = true;
                document.getElementById("dsp-slider-stereo").value = 150;
                document.getElementById("dsp-cb-air").checked = true;
                document.getElementById("dsp-slider-air").value = 12;
            }
            if (track.vocal_density > 0.3) {
                document.getElementById("dsp-cb-vocals").checked = true;
                document.getElementById("dsp-slider-vocals").value = 15;
            }
            if (track.epicness > 0.6) {
                document.getElementById("dsp-cb-bass").checked = true;
                document.getElementById("dsp-slider-bass").value = 10;
                document.getElementById("dsp-cb-limiter").checked = true;
            }
            if (typeof updateLabels === "function") updateLabels();
            if (typeof sendDspUpdate === "function") sendDspUpdate();
            alert("Analysis-driven DSP suggested chain loaded successfully!");
        });
    }
}

// Update active timeline section during playback
let activeSectionIndex = -1;
function updateActiveSectionIndicator(currentTime) {
    if (!state.activeTrackId) return;
    // ... rest of the function
    const timelineOrig = document.getElementById("timeline-container-v2");
    if (!timelineOrig) return;

    const track = trackDetailsCache[state.activeTrackId];
    if (!track) return;

    if (track.section_summary_json && !track._parsed_sections) {
        try {
            track._parsed_sections = JSON.parse(track.section_summary_json);
        } catch(e) {}
    }
    const sections = track.section_summary || track._parsed_sections;
    if (!sections || sections.length === 0) {
        const nameEl = document.getElementById("current-section-name");
        if (nameEl && nameEl.textContent !== "Not Analyzed") {
            nameEl.textContent = "Not Analyzed";
            const timeEl = document.getElementById("current-section-time");
            if (timeEl) timeEl.textContent = "--:-- → --:--";
        }
        return;
    }

    try {
        let foundIdx = -1;

        for (let i = 0; i < sections.length; i++) {
            const start = sections[i].start !== undefined ? sections[i].start : (sections[i].start_time || 0);
            const end = sections[i].end !== undefined ? sections[i].end : (sections[i].end_time || 0);
            if (currentTime >= start && currentTime <= end) {
                foundIdx = i;
                break;
            }
        }

        if (foundIdx !== -1 && foundIdx !== activeSectionIndex) {
            // Update center banner
            const sec = sections[foundIdx];
            const start = sec.start !== undefined ? sec.start : (sec.start_time || 0);
            const end = sec.end !== undefined ? sec.end : (sec.end_time || 0);
            document.getElementById("current-section-name").textContent = sec.label || `Section ${foundIdx + 1}`;
            document.getElementById("current-section-time").textContent = `${formatDuration(start)} → ${formatDuration(end)}`;

            // Highlight section block in drawer timeline using direct ID references
            if (activeSectionIndex !== -1) {
                const prevBlock = document.getElementById(`timeline-sec-${activeSectionIndex}`);
                if (prevBlock) prevBlock.classList.remove("active-block");
            } else {
                const blocks = document.querySelectorAll(".timeline-sec");
                blocks.forEach(b => b.classList.remove("active-block"));
            }
            const activeBlock = document.getElementById(`timeline-sec-${foundIdx}`);
            if (activeBlock) activeBlock.classList.add("active-block");

            // Highlight in fullscreen visualizer using direct ID references
            if (activeSectionIndex !== -1) {
                const prevFsBlock = document.getElementById(`fs-timeline-sec-${activeSectionIndex}`);
                if (prevFsBlock) prevFsBlock.classList.remove("active-block");
            } else {
                const fsBlocks = document.querySelectorAll("[id^='fs-timeline-sec-']");
                fsBlocks.forEach(b => b.classList.remove("active-block"));
            }
            const fsActiveBlock = document.getElementById(`fs-timeline-sec-${foundIdx}`);
            if (fsActiveBlock) fsActiveBlock.classList.add("active-block");

            activeSectionIndex = foundIdx;
        }
    } catch (e) { }
}

function applyLibraryFilter(filterKey, value) {
    // Reset filters to avoid zero matching tracks
    state.searchQuery = "";
    const searchInp = document.getElementById("search-input");
    if (searchInp) searchInp.value = "";

    state.vocalFilter = "";
    document.querySelectorAll("[data-vocal]").forEach(t => {
        if (t.getAttribute("data-vocal") === "") t.classList.add("active");
        else t.classList.remove("active");
    });

    state.characterFilter = "";
    const fChar = document.getElementById("filter-character");
    if (fChar) fChar.value = "";

    state.keyFilter = "";
    const fKey = document.getElementById("filter-musical-key");
    if (fKey) fKey.value = "";

    state.scaleFilter = "";
    const fScale = document.getElementById("filter-major-minor");
    if (fScale) fScale.value = "";

    state.emotionFilter = "";
    const fEmotion = document.getElementById("filter-emotion");
    if (fEmotion) fEmotion.value = "";

    // Reset advanced filters
    const advancedFilters = [
        'strings', 'keyboards', 'piano', 'drums', 'complexity', 'choir', 'guitar', 'bass',
        'winds', 'synth', 'brass',
        'dreaminess', 'epicness', 'cinematicness', 'electronicness', 'nostalgia', 'bpm'
    ];
    advancedFilters.forEach(f => {
        state[f + 'Filter'] = "";
        const el = document.getElementById("filter-" + f);
        if (el) el.value = "";
    });

    // Apply selected filter
    if (filterKey === "vocal") {
        state.vocalFilter = value;
        document.querySelectorAll("[data-vocal]").forEach(t => {
            if (t.getAttribute("data-vocal") === value) t.classList.add("active");
            else t.classList.remove("active");
        });
        saveServerState("player-filter-vocalFilter", value);
    } else if (filterKey === "character") {
        state.characterFilter = value;
        if (fChar) fChar.value = value;
        saveServerState("player-filter-characterFilter", value);
    } else if (filterKey === "key") {
        if (value && value.includes(" ")) {
            const parts = value.split(" ");
            const note = parts[0];
            const scale = parts[1];
            state.keyFilter = note;
            if (fKey) fKey.value = note;
            saveServerState("player-filter-keyFilter", note);

            state.scaleFilter = scale;
            if (fScale) fScale.value = scale;
            saveServerState("player-filter-scaleFilter", scale);
        } else {
            state.keyFilter = value;
            if (fKey) fKey.value = value;
            saveServerState("player-filter-keyFilter", value);
        }
    } else if (filterKey === "scale") {
        state.scaleFilter = value;
        if (fScale) fScale.value = value;
        saveServerState("player-filter-scaleFilter", value);
    } else if (filterKey === "emotion") {
        state.emotionFilter = value;
        if (fEmotion) fEmotion.value = value;
        saveServerState("player-filter-emotionFilter", value);
    } else {
        state[filterKey + 'Filter'] = value;
        const el = document.getElementById("filter-" + filterKey);
        if (el) el.value = value;
        saveServerState("player-filter-" + filterKey + "Filter", value);

        const advancedPanel = document.getElementById("advanced-filters-panel");
        if (advancedPanel) advancedPanel.classList.add("active");
    }

    if (typeof window.updateAdvancedFiltersBadge === "function") {
        window.updateAdvancedFiltersBadge();
    }

    // Switch workspace to Library Explorer
    const libLink = document.querySelector(".sidebar-nav a[data-workspace='workspace-library']");
    if (libLink) {
        libLink.click();
    } else {
        document.querySelectorAll(".workspace-panel").forEach(panel => {
            panel.style.display = "none";
            panel.classList.remove("active");
        });
        const libPanel = document.getElementById("workspace-library");
        if (libPanel) {
            libPanel.style.display = "block";
            libPanel.classList.add("active");
        }
        state.activeWorkspace = "workspace-library";
        saveServerState("player-active-workspace", "workspace-library");
    }

    state.currentPage = 1;
    loadTracks();
}

// Dynamic Vibe-to-Color Theme mapping helper
function updateDynamicTheming(track) {
    if (!track) return;
    
    // Standard Fallback Accent Colors (Emerald Mint + Cyan)
    let primary = '#10b981';
    let primaryGlow = 'rgba(16, 185, 129, 0.25)';
    let secondary = '#34d399';
    let secondaryGlow = 'rgba(52, 211, 153, 0.25)';
    let bubbleColor = 'rgba(16, 185, 129, 0.15)';
    
    const epicness = parseFloat(track.epicness || 0);
    const dreaminess = parseFloat(track.dreaminess || 0);
    const character = track.audio_character || "";
    const emotion = track.emotion_primary || "";
    
    if (epicness > 0.6) {
        // Epic & Intense Vibe Profile -> Signal Orange / Crimson Red
        primary = '#ff6b00';
        primaryGlow = 'rgba(255, 107, 0, 0.25)';
        secondary = '#f43f5e';
        secondaryGlow = 'rgba(244, 63, 94, 0.25)';
        bubbleColor = 'rgba(255, 107, 0, 0.15)';
    } else if (dreaminess > 0.6) {
        // Dreamy / Cosmic Vibe Profile -> Mint Green / Soft Teal
        primary = '#34d399';
        primaryGlow = 'rgba(52, 211, 153, 0.25)';
        secondary = '#22d3ee';
        secondaryGlow = 'rgba(34, 211, 238, 0.25)';
        bubbleColor = 'rgba(52, 211, 153, 0.15)';
    } else if (character.includes('Calm') || character.includes('Smooth')) {
        // Calm / Serene Vibe Profile -> Studio Slate Blue / Sky Blue
        primary = '#3b82f6';
        primaryGlow = 'rgba(59, 130, 246, 0.25)';
        secondary = '#60a5fa';
        secondaryGlow = 'rgba(96, 165, 250, 0.25)';
        bubbleColor = 'rgba(59, 130, 246, 0.15)';
    } else if (emotion.includes('Dark') || emotion.includes('Sad')) {
        // Dark / Melancholy Vibe Profile -> Muted Slate Grey / Abyssal Red
        primary = '#64748b';
        primaryGlow = 'rgba(100, 116, 139, 0.25)';
        secondary = '#991b1b';
        secondaryGlow = 'rgba(153, 27, 27, 0.25)';
        bubbleColor = 'rgba(153, 27, 27, 0.12)';
    }
    
    // Apply variables to document roots to trigger immediate transition rules
    document.documentElement.style.setProperty('--accent-purple', primary);
    document.documentElement.style.setProperty('--accent-purple-glow', primaryGlow);
    document.documentElement.style.setProperty('--accent-cyan', secondary);
    document.documentElement.style.setProperty('--accent-cyan-glow', secondaryGlow);
    document.documentElement.style.setProperty('--accent-emerald-glow', bubbleColor);
}

// Track details loading (Details Drawer)
// Track details loading (Details Drawer)
async function selectTrack(trackId, autoPlay = true) {
    state.activeTrackId = trackId !== null && trackId !== undefined ? Number(trackId) : null;
    lastActiveLyricIdx = -1;
    activeSectionIndex = -1;

    // Cache the active track's album tracks for Repeat Mode: Album
    state.currentAlbumTracks = [];
    if (trackId !== null) {
        const currentTrack = state.activePlaylist.find(t => Number(t.id) === Number(trackId));
        const albumName = currentTrack ? currentTrack.album : "";
        if (albumName) {
            fetch(`/api/remote/tracks?album=${encodeURIComponent(albumName)}`)
                .then(r => r.json())
                .then(tracks => {
                    state.currentAlbumTracks = tracks || [];
                }).catch(err => console.error("Error caching album tracks:", err));
        } else {
            fetch(`/api/track?id=${trackId}`)
                .then(res => res.json())
                .then(track => {
                    if (track && track.album) {
                        fetch(`/api/remote/tracks?album=${encodeURIComponent(track.album)}`)
                            .then(r => r.json())
                            .then(tracks => {
                                state.currentAlbumTracks = tracks || [];
                            }).catch(err => console.error("Error caching album tracks:", err));
                    }
                }).catch(err => console.error("Error fetching track details for album cache:", err));
        }
    }



    const nameEl = document.getElementById("current-section-name");
    if (nameEl) nameEl.textContent = "-";
    const timeEl = document.getElementById("current-section-time");
    if (timeEl) timeEl.textContent = "00:00 → 00:00";

    // Save last track to localStorage
    saveServerState("player-last-track-id", trackId);
    if (autoPlay) {
        try {
            const res = await fetch("/api/player/play_id", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ id: trackId }) });
            if (res.status === 423) {
                alert("This track is currently locked because it is being processed by a background task (e.g. Translation or Lyrics Sync). Please wait a moment and try again.");
                return;
            }
        } catch (e) {
            console.error(e);
        }
    }

    // Highlight active row in Explorer table or grid if it exists
    const items = document.querySelectorAll("#workspace-library #tracks-tbody tr, #workspace-library #tracks-grid .grid-card");
    let activeRow = null;
    items.forEach(r => {
        if (parseInt(r.getAttribute("data-id")) === trackId) {
            r.classList.add("active-row");
            activeRow = r;
            // Add EQ animation wrapper if it doesn't exist
            let titleEl = r.querySelector(".title-col") || r.querySelector(".grid-card-title");
            if (titleEl && !titleEl.querySelector(".now-playing-eq")) {
                titleEl.insertAdjacentHTML("beforeend", `<span class="now-playing-eq" style="display:inline-flex; margin-left:6px; vertical-align:middle; flex-shrink:0;"><span class="eq-bar"></span><span class="eq-bar"></span><span class="eq-bar"></span><span class="eq-bar"></span></span>`);
            }
        } else {
            r.classList.remove("active-row");
            let eq = r.querySelector(".now-playing-eq");
            if (eq) eq.remove();
        }
    });
    if (activeRow) {
        activeRow.scrollIntoView({ behavior: "smooth", block: "nearest" });
    }

    // Update queue workspace highlight
    loadQueueWorkspace();

    // Slide open details drawer only if not collapsed
    const container = document.querySelector(".app-container");
    const wasCollapsed = container ? container.classList.contains("player-collapsed") : false;

    if (!wasCollapsed) {
        if (detailsDrawer) detailsDrawer.classList.add("drawer-open");
        if (container) {
            container.classList.add("drawer-open");
            container.classList.remove("player-collapsed");
        }
    } else {
        if (detailsDrawer) detailsDrawer.classList.remove("drawer-open");
        if (container) {
            container.classList.remove("drawer-open");
            container.classList.add("player-collapsed");
        }
    }

    const savedPanelWidth = (window.serverState?.preferences?.["player-panel-width"]) || "380";
    if (detailsDrawer) detailsDrawer.style.width = `${savedPanelWidth}px`;
    const btnTogglePlayer = document.getElementById("btn-toggle-player");
    if (btnTogglePlayer) {
        if (wasCollapsed) {
            btnTogglePlayer.classList.add("collapsed-active");
        } else {
            btnTogglePlayer.classList.remove("collapsed-active");
        }
    }

    // Client cache fetch
    let track;
    const now = Date.now();
    if (trackDetailsCache[trackId] && trackDetailsTimestamp[trackId] && (now - trackDetailsTimestamp[trackId] < 300000)) {
        track = trackDetailsCache[trackId];
    } else {
        try {
            const res = await fetch(`/api/track?id=${trackId}`);
            if (!res.ok) throw new Error("Track details fetch failed");
            track = await res.json();
            trackDetailsCache[trackId] = track; // cache details
            trackDetailsTimestamp[trackId] = now;
        } catch (err) {
            console.error("Error loading track details:", err);
            return;
        }
    }

    state.activeTrackId = trackId;
    updateAudioQualityPillBanner();

    // Dynamic Vibe Theming Accent Update
    updateDynamicTheming(track);

    // Set up Media Session Metadata and Sync Remote Control
    if ('mediaSession' in navigator && track) {
        navigator.mediaSession.metadata = new MediaMetadata({
            title: track.title,
            artist: track.artist,
            album: track.album || "Unknown Album",
            artwork: [{ src: `/api/art?id=${track.album_art_id || track.id}`, sizes: '512x512', type: 'image/jpeg' }]
        });
    }
    // Sync queue to backend to keep Windows SMTC OS keys in sync
    fetch("/api/player/queue", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ queue: state.activePlaylist.map(tr => tr.id), start_track_id: trackId })
    }).catch(err => console.error("Error syncing queue to backend:", err));

    syncRemoteControlStatus();

    // Show drawer content, hide empty state
    drawerEmpty.style.display = "none";
    drawerInner.style.display = "flex";

    // Render headers
    document.getElementById("drawer-title").textContent = track.title;

    // Update Mini Player track info
    const miniTitle = document.getElementById("mini-player-title");
    const miniArtist = document.getElementById("mini-player-artist");
    const miniArt = document.getElementById("mini-player-art");

    if (miniTitle) miniTitle.textContent = track.title;
    if (miniArtist) miniArtist.textContent = track.artist;
    if (miniArt) {
        miniArt.src = `/api/art?id=${track.album_art_id || trackId}`;
    }

    const drawerArtist = document.getElementById("drawer-artist");
    if (drawerArtist) {
        drawerArtist.innerHTML = renderArtistLinks(track.artist, false);
        drawerArtist.onclick = null;
    }

    const drawerAlbum = document.getElementById("drawer-album");
    if (drawerAlbum) {
        drawerAlbum.innerHTML = `<i class="fa-solid fa-compact-disc"></i> ${escapeHtml(track.album)}`;
        drawerAlbum.onclick = () => { filterByAlbum(track.album); };
    }

    // Favorite Star sync
    const drawerStar = document.getElementById("btn-drawer-favorite");
    if (drawerStar) {
        const icon = drawerStar.querySelector("i");
        if (track.favorite_count > 0) {
            drawerStar.classList.add("favorited");
            if (icon) icon.className = "fa-solid fa-star";
        } else {
            drawerStar.classList.remove("favorited");
            if (icon) icon.className = "fa-regular fa-star";
        }
        const newStar = drawerStar.cloneNode(true);
        drawerStar.parentNode.replaceChild(newStar, drawerStar);
        newStar.addEventListener("click", () => { toggleFavorite(trackId, newStar); });
    }

    const miniStar = document.getElementById("mini-btn-favorite");
    if (miniStar) {
        const icon = miniStar.querySelector("i");
        if (track.favorite_count > 0) {
            miniStar.classList.add("favorited");
            if (icon) icon.className = "fa-solid fa-star";
        } else {
            miniStar.classList.remove("favorited");
            if (icon) icon.className = "fa-regular fa-star";
        }
    }

    // Dislike Button sync
    const drawerDislike = document.getElementById("btn-drawer-dislike");
    if (drawerDislike) {
        const icon = drawerDislike.querySelector("i");
        if (track.disliked > 0) {
            drawerDislike.classList.add("disliked");
            drawerDislike.style.color = "var(--accent-cyan)";
            if (icon) icon.className = "fa-solid fa-thumbs-down";
        } else {
            drawerDislike.classList.remove("disliked");
            drawerDislike.style.color = "var(--text-muted)";
            if (icon) icon.className = "fa-regular fa-thumbs-down";
        }
        const newDislike = drawerDislike.cloneNode(true);
        drawerDislike.parentNode.replaceChild(newDislike, drawerDislike);
        newDislike.addEventListener("click", () => { toggleDislike(trackId); });
    }

    const miniDislike = document.getElementById("mini-btn-dislike");
    if (miniDislike) {
        const icon = miniDislike.querySelector("i");
        if (track.disliked > 0) {
            miniDislike.classList.add("disliked");
            miniDislike.style.color = "var(--accent-cyan)";
            if (icon) icon.className = "fa-solid fa-thumbs-down";
        } else {
            miniDislike.classList.remove("disliked");
            miniDislike.style.color = "var(--text-muted)";
            if (icon) icon.className = "fa-regular fa-thumbs-down";
        }
    }

    const initials = track.title.substring(0, 2);
    const initialsSpan = document.getElementById("drawer-album-initials");
    const artImg = document.getElementById("drawer-album-art");

    if (initialsSpan && artImg) {
        initialsSpan.textContent = initials;
        initialsSpan.style.display = "inline";
        artImg.style.display = "none";
        artImg.src = "";
        const artUrl = `/api/art?id=${track.album_art_id || trackId}`;
        const tempImg = new Image();
        tempImg.onload = () => {
            artImg.src = artUrl;
            artImg.style.display = "block";
            initialsSpan.style.display = "none";
        };
        tempImg.onerror = () => {
            artImg.style.display = "none";
            initialsSpan.style.display = "inline";
        };
        tempImg.src = artUrl;
    }

    // Wire album art click → jump to track in Library Explorer
    const artClickTarget = document.querySelector(".album-art-placeholder");
    if (artClickTarget) {
        artClickTarget.style.cursor = "pointer";
        artClickTarget.title = "Go to track in Library Explorer";
        artClickTarget.onclick = () => jumpToTrackInExplorer(trackId);
    }

    // Render specs and visualizations
    renderIntelligenceSummary(track);
    renderRadarChart(track, "chart-radar-container");
    renderEmotionArcChart(track, "chart-emotion-container");
    renderSectionTimeline(track);
    renderInstrumentHeatmap(track);
    renderProfilerGrid(track);
    renderRecommendations(track.similar_tracks);
    renderThemeVariations(track.theme_variations);
    renderDSPRecommendations(track);

    // Setup peak jump buttons
    const setupPeakJumpBtn = (btnId) => {
        const btn = document.getElementById(btnId);
        if (btn) {
            const newBtn = btn.cloneNode(true);
            btn.parentNode.replaceChild(newBtn, btn);
            newBtn.addEventListener("click", () => {
                if (track.peak_timestamp !== null && track.peak_timestamp !== undefined) {
                    seekAudioTo(track.peak_timestamp);
                } else {
                    alert("This track has not been fully analyzed for peak moments.");
                }
            });
        }
    };
    setupPeakJumpBtn("audio-btn-jump-peak");
    setupPeakJumpBtn("audio-btn-jump-peak-controls");

    // Populate Lyrics
    const lyricsText = document.getElementById("lyrics-content");
    state.lyricLines = [];
    try {
        const res = await fetch(`/api/track/lyrics?id=${track.id}`);
        if (res.ok) {
            const lyricsArray = await res.json();
            let parsedHtml = "";
            let lineIdx = 0;
            lyricsArray.forEach(line => {
                const time = line.time;
                const text = line.text.trim();
                state.lyricLines.push({ time, text, index: lineIdx });
                parsedHtml += `<div class="lyrics-line" id="lyric-line-${lineIdx}" data-time="${time}">${escapeHtml(text || "🎵")}</div>`;
                lineIdx++;
            });
            if (lyricsText) lyricsText.innerHTML = parsedHtml || `<div class="no-lyrics">No lyrics available for this track.</div>`;
        } else {
            if (lyricsText) lyricsText.innerHTML = `<div class="no-lyrics">No lyrics available for this track.</div>`;
        }
    } catch (e) {
        if (lyricsText) lyricsText.innerHTML = `<div class="no-lyrics">No lyrics available for this track.</div>`;
    }

    // Sync queue widget list
    updateQueueWidget();

    // Play automatically if requested
    if (autoPlay) {
        playAudio();
    }

    // Force active presentation sync if overlay is active
    const fsOverlay = document.getElementById("fullscreen-overlay");
    if (fsOverlay && (fsOverlay.style.display === "flex" || fsOverlay.classList.contains("fs-active"))) {
        syncFullscreenVisualizer(track);
    }

    // Render Similar Tracks
    const similarContainer = document.getElementById("drawer-similar-list");
    if (similarContainer && track.similar_tracks) {
        similarContainer.innerHTML = "";
        track.similar_tracks.slice(0, 10).forEach(sim => {
            const el = document.createElement("div");
            el.className = "similar-track-item";
            el.innerHTML = `
                <div style="display: flex; align-items: center; gap: 10px;">
                    <img src="/api/art?id=${sim.album_art_id || sim.id}" style="width: 40px; height: 40px; border-radius: 4px; object-fit: cover;">
                    <div style="flex: 1; min-width: 0;">
                        <div style="font-weight: 600; font-size: 13px; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">${escapeHtml(sim.title)}</div>
                        <div style="font-size: 11px; color: var(--text-muted);">${escapeHtml(sim.artist)}</div>
                    </div>
                    <div style="font-size: 11px; color: var(--accent-magenta); font-weight: bold;">${(sim.similarity * 100).toFixed(1)}%</div>
                </div>
            `;
            el.addEventListener("click", async () => {
                let currentIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(state.activeTrackId));
                if (currentIdx === -1) {
                    state.activePlaylist = [{ id: sim.id, title: sim.title, artist: sim.artist, album: sim.album || "", duration: sim.duration || 180 }];
                    currentIdx = 0;
                } else {
                    const simTrackIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(sim.id));
                    if (simTrackIdx === -1) {
                        const simTrackObj = { id: sim.id, title: sim.title, artist: sim.artist, album: sim.album || "", duration: sim.duration || 180 };
                        state.activePlaylist.splice(currentIdx + 1, 0, simTrackObj);
                    }
                }

                state.shuffleIndices = Array.from({ length: state.activePlaylist.length }, (_, i) => i);

                await selectTrack(sim.id, true);

                await fetch("/api/player/queue", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({
                        queue: state.activePlaylist.map(tr => tr.id),
                        start_track_id: sim.id
                    })
                });
            });
            similarContainer.appendChild(el);
        });
    }

    // Render DTW Melody Matches
    const melodyContainer = document.getElementById("drawer-melody-list");
    if (melodyContainer) {
        melodyContainer.innerHTML = `<span class="no-themes">Loading melody twins...</span>`;
        fetch(`/api/track/melody_matches?id=${trackId}`)
            .then(res => res.json())
            .then(data => {
                if (data.matches && data.matches.length > 0) {
                    melodyContainer.innerHTML = "";
                    data.matches.forEach(mel => {
                        const el = document.createElement("div");
                        el.className = "similar-track-item";
                        el.style.borderLeft = "3px solid var(--accent-cyan)";
                        el.innerHTML = `
                            <div style="display: flex; align-items: center; gap: 10px;">
                                <img src="/api/art?id=${mel.album_art_id || mel.id}" style="width: 40px; height: 40px; border-radius: 4px; object-fit: cover;">
                                <div style="flex: 1; min-width: 0;">
                                    <div style="font-weight: 600; font-size: 13px; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">${escapeHtml(mel.title)}</div>
                                    <div style="font-size: 11px; color: var(--text-muted);">${escapeHtml(mel.artist)}</div>
                                </div>
                                <div style="font-size: 11px; color: var(--accent-cyan); font-weight: bold;">${(typeof mel.similarity === "number") ? mel.similarity.toFixed(1) : ((typeof mel.score === "number") ? (mel.score * 100).toFixed(1) : "95.0")}%</div>
                            </div>
                        `;
                        el.addEventListener("click", async () => {
                            let currentIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(state.activeTrackId));
                            if (currentIdx === -1) {
                                state.activePlaylist = [{ id: mel.id, title: mel.title, artist: mel.artist, album: mel.album || "", duration: mel.duration || 180 }];
                                currentIdx = 0;
                            } else {
                                const simTrackIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(mel.id));
                                if (simTrackIdx === -1) {
                                    const simTrackObj = { id: mel.id, title: mel.title, artist: mel.artist, album: mel.album || "", duration: mel.duration || 180 };
                                    state.activePlaylist.splice(currentIdx + 1, 0, simTrackObj);
                                }
                            }
                            state.shuffleIndices = Array.from({ length: state.activePlaylist.length }, (_, i) => i);
                            await selectTrack(mel.id, true);
                            await fetch("/api/player/queue", {
                                method: "POST",
                                headers: { "Content-Type": "application/json" },
                                body: JSON.stringify({
                                    queue: state.activePlaylist.map(tr => tr.id),
                                    start_track_id: mel.id
                                })
                            });
                        });
                        melodyContainer.appendChild(el);
                    });
                } else {
                    melodyContainer.innerHTML = `<span class="no-themes">No melody twins found in cache for this track.</span>`;
                }
            })
            .catch(err => {
                console.error("Error fetching melody matches:", err);
                melodyContainer.innerHTML = `<span class="no-themes">Error loading melody matches.</span>`;
            });
    }
}


// Play next track or page advance
function playImmediate(trackId) {
    selectTrack(trackId, true);
}

// Custom Audio Player setup & actions
function setupAudioPlayer() {
    playPauseBtn.addEventListener("click", () => {
        if (state.isPlaying) {
            pauseAudio();
        } else {
            playAudio();
        }
    });

    rewindBtn.addEventListener("click", () => playPreviousTrack());
    forwardBtn.addEventListener("click", () => playNextTrack());

    // Seek slider: update UI during drag, send seek to backend on release
    audioSlider.addEventListener("input", () => {
        if (!state.duration) return;
        const targetSec = (audioSlider.value / 100) * state.duration;
        audioTimeCurrent.textContent = formatDuration(targetSec);
        audioSlider.style.setProperty("--progress", `${audioSlider.value}%`);
        audioSlider.dataset.dragging = "true";
    });

    audioSlider.addEventListener("change", () => {
        if (!state.duration) return;
        const targetSec = (audioSlider.value / 100) * state.duration;
        // Pin the rAF clock to the seeked position immediately
        state.baseCurrentTimeSec = targetSec;
        state.basePerfTime = performance.now();
        state.localPlayTimeSec = targetSec;
        state.lastSeekTimestamp = Date.now();
        fetch("/api/player/seek", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ time: targetSec }) });
        delete audioSlider.dataset.dragging;
        // Force fill bar to seeked position instantly (no bounce)
        lastRenderedProgress = -1;
        if (state.isPlaying) startProgressAnimation();
    });

    let rtcPeerConn = null;
    let rtcDataChannel = null;

    async function initWebRTCDataChannel() {
        if (!('RTCPeerConnection' in window)) return;
        try {
            rtcPeerConn = new RTCPeerConnection({
                iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
            });
            rtcDataChannel = rtcPeerConn.createDataChannel('control');

            rtcDataChannel.onopen = () => {
                console.log('[WebRTC] Sub-10ms P2P UDP DataChannel connected & active!');
            };

            const offer = await rtcPeerConn.createOffer();
            await rtcPeerConn.setLocalDescription(offer);

            const res = await fetch('/api/webrtc/offer', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ sdp: offer.sdp, type: offer.type })
            });
            if (res.ok) {
                const answer = await res.json();
                if (answer && answer.sdp) {
                    await rtcPeerConn.setRemoteDescription(new RTCSessionDescription(answer));
                }
            }
        } catch (err) {
            console.log('[WebRTC] DataChannel fallback to WebSocket:', err);
        }
    }
    setTimeout(initWebRTCDataChannel, 1000);

    // Ultra-responsive WebRTC/WebSocket volume throttle helpers
    let lastWsAudioVolSend = 0;
    let pendingWsAudioVol = null;
    let wsAudioVolTimer = null;

    function sendAudioVolumeWsThrottled(val) {
        pendingWsAudioVol = val;
        const now = Date.now();
        if (now - lastWsAudioVolSend >= 25) {
            lastWsAudioVolSend = now;
            if (wsAudioVolTimer) { clearTimeout(wsAudioVolTimer); wsAudioVolTimer = null; }
            if (rtcDataChannel && rtcDataChannel.readyState === 'open') {
                rtcDataChannel.send(`volume:${val}`);
            } else if (isWsConnected && ws && ws.readyState === WebSocket.OPEN) {
                ws.send(JSON.stringify({ type: "command", command: `volume:${val}` }));
            } else {
                fetch("/api/player/volume", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ volume: val }) }).catch(() => {});
            }
        } else if (!wsAudioVolTimer) {
            wsAudioVolTimer = setTimeout(() => {
                if (pendingWsAudioVol !== null) {
                    sendAudioVolumeWsThrottled(pendingWsAudioVol);
                }
            }, 25);
        }
    }

    let lastWsSysVolSend = 0;
    let pendingWsSysVol = null;
    let wsSysVolTimer = null;

    function sendSystemVolumeWsThrottled(val) {
        pendingWsSysVol = val;
        const now = Date.now();
        if (now - lastWsSysVolSend >= 25) {
            lastWsSysVolSend = now;
            if (wsSysVolTimer) { clearTimeout(wsSysVolTimer); wsSysVolTimer = null; }
            if (rtcDataChannel && rtcDataChannel.readyState === 'open') {
                rtcDataChannel.send(`system_volume:${val}`);
            } else if (isWsConnected && ws && ws.readyState === WebSocket.OPEN) {
                ws.send(JSON.stringify({ type: "command", command: `system_volume:${val}` }));
            } else {
                fetch("/api/system/volume", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ volume: val }) }).catch(() => {});
            }
        } else if (!wsSysVolTimer) {
            wsSysVolTimer = setTimeout(() => {
                if (pendingWsSysVol !== null) {
                    sendSystemVolumeWsThrottled(pendingWsSysVol);
                }
            }, 25);
        }
    }

    function syncAllAudioVolumeUI(val) {
        val = Math.max(0, Math.min(100, val));
        state.volume = val;
        updateVolumeIcon(val / 100);

        const volSlider = document.getElementById("audio-volume-slider");
        const miniVolumeSlider = document.getElementById("mini-audio-volume-slider");
        const overlayVolSlider = document.getElementById("overlay-volume-slider");
        const audioVolVal = document.getElementById("audio-volume-val");
        const remoteLabel = document.getElementById("remote-volume-label");

        if (volSlider) { volSlider.value = val; volSlider.style.setProperty("--progress", `${val}%`); }
        if (miniVolumeSlider) { miniVolumeSlider.value = val; miniVolumeSlider.style.setProperty("--progress", `${val}%`); }
        if (overlayVolSlider) { overlayVolSlider.value = val; overlayVolSlider.style.setProperty("--progress", `${val}%`); }
        if (audioVolVal) audioVolVal.textContent = `${val}%`;
        if (remoteLabel) remoteLabel.textContent = `${val}%`;

        sendAudioVolumeWsThrottled(val);

        if (val > 0) {
            state.lastVolume = val;
            saveServerState("player-last-volume", val);
        }
        saveServerState("player-volume", val);
    }

    function syncAllSystemVolumeUI(val) {
        val = Math.max(0, Math.min(100, val));
        const sysSlider = document.getElementById("system-volume-slider");
        const miniSysSlider = document.getElementById("mini-system-volume-slider");
        const overlaySysVolSlider = document.getElementById("overlay-system-volume-slider");
        const sysValSpan = document.getElementById("system-volume-val");
        const overlaySysLabel = document.getElementById("overlay-system-volume-label");

        if (sysSlider) { sysSlider.value = val; sysSlider.style.setProperty("--sys-progress", `${val}%`); }
        if (miniSysSlider) { miniSysSlider.value = val; miniSysSlider.style.setProperty("--sys-progress", `${val}%`); }
        if (overlaySysVolSlider) { overlaySysVolSlider.value = val; overlaySysVolSlider.style.setProperty("--sys-progress", `${val}%`); }
        if (sysValSpan) sysValSpan.textContent = `${val}%`;
        if (overlaySysLabel) overlaySysLabel.textContent = `${val}%`;

        sendSystemVolumeWsThrottled(val);
    }

    // Volume controls
    volumeSlider.addEventListener("input", () => {
        const val = parseInt(volumeSlider.value) || 0;
        syncAllAudioVolumeUI(val);
    });

    const miniVolumeSlider = document.getElementById("mini-audio-volume-slider");
    if (miniVolumeSlider) {
        miniVolumeSlider.addEventListener("input", () => {
            const val = parseInt(miniVolumeSlider.value) || 0;
            syncAllAudioVolumeUI(val);
        });
    }

    const overlayVolSlider = document.getElementById("overlay-volume-slider");
    if (overlayVolSlider) {
        overlayVolSlider.addEventListener("input", () => {
            const val = parseInt(overlayVolSlider.value) || 0;
            syncAllAudioVolumeUI(val);
        });
    }

    const systemSlider = document.getElementById("system-volume-slider");
    if (systemSlider) {
        systemSlider.addEventListener("input", () => {
            const val = parseInt(systemSlider.value) || 0;
            syncAllSystemVolumeUI(val);
        });
    }

    const miniSysVolSlider = document.getElementById("mini-system-volume-slider");
    if (miniSysVolSlider) {
        miniSysVolSlider.addEventListener("input", () => {
            const val = parseInt(miniSysVolSlider.value) || 0;
            syncAllSystemVolumeUI(val);
        });
    }

    const overlaySysVolSlider = document.getElementById("overlay-system-volume-slider");
    if (overlaySysVolSlider) {
        overlaySysVolSlider.addEventListener("input", () => {
            const val = parseInt(overlaySysVolSlider.value) || 0;
            syncAllSystemVolumeUI(val);
        });
    }

    volumeIcon.addEventListener("click", () => {
        const currentVal = parseInt(volumeSlider.value);
        if (currentVal > 0) {
            state.lastVolume = currentVal;
            saveServerState("player-last-volume", currentVal);
            volumeSlider.value = 0;
            updateVolumeIcon(0);
            fetch("/api/player/volume", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ volume: 0 }) });
        } else {
            const restoreVal = state.lastVolume || 80;
            volumeSlider.value = restoreVal;
            updateVolumeIcon(restoreVal / 100);
            fetch("/api/player/volume", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ volume: restoreVal }) });
        }
    });

    // Mini Player Controls
    const miniPlayBtn = document.getElementById("mini-btn-play");
    const miniPrevBtn = document.getElementById("mini-btn-prev");
    const miniNextBtn = document.getElementById("mini-btn-next");
    const miniProgressTrigger = document.getElementById("mini-progress-bar-trigger");

    if (miniPlayBtn) {
        miniPlayBtn.addEventListener("click", () => {
            if (state.isPlaying) {
                pauseAudio();
            } else {
                playAudio();
            }
        });
    }
    if (miniPrevBtn) {
        miniPrevBtn.addEventListener("click", () => playPreviousTrack());
    }
    if (miniNextBtn) {
        miniNextBtn.addEventListener("click", () => playNextTrack());
    }
    if (miniProgressTrigger) {
        miniProgressTrigger.addEventListener("click", (e) => {
            if (!state.duration) return;
            const rect = miniProgressTrigger.getBoundingClientRect();
            const percent = (e.clientX - rect.left) / rect.width;
            const targetSec = percent * state.duration;
            fetch("/api/player/seek", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ time: targetSec }) });
        });
    }

    // Mini Player Shuffle, Repeat, Favorite, Dislike Controls
    const miniShuffleBtn = document.getElementById("mini-btn-shuffle");
    const miniShuffleModeBtn = document.getElementById("mini-btn-shuffle-mode");
    const miniRepeatBtn = document.getElementById("mini-btn-repeat");
    const miniRepeatModeBtn = document.getElementById("mini-btn-repeat-mode");
    const miniFavoriteBtn = document.getElementById("mini-btn-favorite");
    const miniDislikeBtn = document.getElementById("mini-btn-dislike");

    if (miniShuffleBtn && !miniShuffleBtn.dataset.bound) {
        miniShuffleBtn.dataset.bound = "true";
        miniShuffleBtn.addEventListener("click", (e) => { e.stopPropagation(); toggleShuffleState(); });
    }
    if (miniShuffleModeBtn && !miniShuffleModeBtn.dataset.bound) {
        miniShuffleModeBtn.dataset.bound = "true";
        miniShuffleModeBtn.addEventListener("click", (e) => { e.stopPropagation(); cycleShuffleMode(); });
    }
    if (miniRepeatBtn && !miniRepeatBtn.dataset.bound) {
        miniRepeatBtn.dataset.bound = "true";
        miniRepeatBtn.addEventListener("click", (e) => { e.stopPropagation(); toggleRepeatState(); });
    }
    if (miniRepeatModeBtn && !miniRepeatModeBtn.dataset.bound) {
        miniRepeatModeBtn.dataset.bound = "true";
        miniRepeatModeBtn.addEventListener("click", (e) => { e.stopPropagation(); cycleRepeatMode(); });
    }
    if (miniFavoriteBtn && !miniFavoriteBtn.dataset.bound) {
        miniFavoriteBtn.dataset.bound = "true";
        miniFavoriteBtn.addEventListener("click", (e) => {
            e.stopPropagation();
            if (state.activeTrackId) toggleFavorite(state.activeTrackId, miniFavoriteBtn);
        });
    }
    if (miniDislikeBtn && !miniDislikeBtn.dataset.bound) {
        miniDislikeBtn.dataset.bound = "true";
        miniDislikeBtn.addEventListener("click", (e) => {
            e.stopPropagation();
            if (state.activeTrackId) toggleDislike(state.activeTrackId, miniDislikeBtn);
        });
    }

    // Start polling the backend for player status
    startPlayerStatusPolling();
}
let statusPollInterval = null;
let ws = null;
let isWsConnected = false;

async function processPlayerStatus(status) {
    // 1. Update Play/Pause button state
    if (state.isPlaying !== status.is_playing) {
        state.isPlaying = status.is_playing;
        playPauseBtn.innerHTML = state.isPlaying ? `<i class="fa-solid fa-pause"></i>` : `<i class="fa-solid fa-play"></i>`;
        const fsBtn = document.getElementById("fs-btn-play-pause");
        if (fsBtn) fsBtn.innerHTML = state.isPlaying ? `<i class="fa-solid fa-pause"></i>` : `<i class="fa-solid fa-play"></i>`;

        const miniPlayBtn = document.getElementById("mini-btn-play");
        if (miniPlayBtn) {
            miniPlayBtn.innerHTML = state.isPlaying ? `<i class="fa-solid fa-pause"></i>` : `<i class="fa-solid fa-play"></i>`;
        }
        if (state.isPlaying) {
            startProgressAnimation();
        }
    }
    document.body.classList.toggle("music-playing", state.isPlaying);

    // 2. Update Progress Bar state reference
    const currentSec = status.current_time_ms / 1000;
    let totalSec = status.duration_ms / 1000;
    if (totalSec <= 0 && state.activeTrackId && trackDetailsCache[state.activeTrackId]) {
        totalSec = trackDetailsCache[state.activeTrackId].duration || 0;
    }
    state.duration = totalSec;

    // SEEK LOCK-OUT: ignore stale server position for 1500ms after user seeks
    const msSinceLastSeek = state.lastSeekTimestamp ? (Date.now() - state.lastSeekTimestamp) : Infinity;
    if (msSinceLastSeek > 1500) {
        // Normal drift correction — only if no recent seek
        const currentInterpolated = (state.baseCurrentTimeSec || 0) + ((performance.now() - (state.basePerfTime || performance.now())) / 1000);
        const drift = Math.abs(currentInterpolated - currentSec);

        if (drift > 0.4 || !state.isPlaying) {
            state.baseCurrentTimeSec = currentSec;
            state.basePerfTime = performance.now();
            state.localPlayTimeSec = currentSec;
        }
    }
    // (else: within seek lock-out window — keep basePerfTime/baseCurrentTimeSec at seeked position)

    // Sync active heatmap cell highlight & Live Chord Display
    syncHeatmapHighlight(state.localPlayTimeSec, totalSec);

    const audioTimeTotalEl = document.getElementById("audio-time-total");
    if (audioTimeTotalEl) audioTimeTotalEl.textContent = formatDuration(totalSec);

    const miniTimeTotal = document.getElementById("mini-time-total");
    if (miniTimeTotal) miniTimeTotal.textContent = formatDuration(totalSec);

    // Sync active lyrics & section details during playback
    updateLyricsSync(state.localPlayTimeSec);
    updateActiveSectionIndicator(state.localPlayTimeSec);

    // 3. Shuffle/Repeat state sync from server status
    if (status.shuffle_mode !== undefined && status.shuffle_mode !== state.shuffleMode) {
        state.shuffleMode = status.shuffle_mode || false;
        syncShuffleModeUI();
    }
    if (status.repeat_mode !== undefined && status.repeat_mode !== state.repeatMode) {
        state.repeatMode = status.repeat_mode || "none";
        syncRepeatModeUI();
    }

    // Queue sync: now handled by WS "queue" message push (no HTTP fetch needed)
    if (status.queue_version !== undefined && status.queue_version !== state.queueVersion) {
        state.queueVersion = status.queue_version;
        // Queue data arrives via WS "queue" push — nothing to do here
    }

    // 3. Audio Volume & System Volume Sync
    if (status.volume !== undefined) {
        const volVal = Math.round(Number(status.volume) || 0);
        state.volume = volVal;
        const volSlider = document.getElementById("audio-volume-slider");
        const miniVolSlider = document.getElementById("mini-audio-volume-slider");
        const overlayVolSlider = document.getElementById("overlay-volume-slider");
        const volPctSpan = document.getElementById("audio-volume-val");
        const remoteVolLabel = document.getElementById("remote-volume-label");

        if (volSlider && document.activeElement !== volSlider) {
            volSlider.value = volVal;
            volSlider.style.setProperty("--progress", `${volVal}%`);
            updateVolumeIcon(volVal / 100);
        }
        if (miniVolSlider && document.activeElement !== miniVolSlider) {
            miniVolSlider.value = volVal;
            miniVolSlider.style.setProperty("--progress", `${volVal}%`);
        }
        if (overlayVolSlider && document.activeElement !== overlayVolSlider) {
            overlayVolSlider.value = volVal;
            overlayVolSlider.style.setProperty("--progress", `${volVal}%`);
        }
        if (volPctSpan) volPctSpan.textContent = `${volVal}%`;
        if (remoteVolLabel) remoteVolLabel.textContent = `${volVal}%`;
    }

    if (status.system_volume !== undefined) {
        const sysVal = Math.round(Number(status.system_volume) || 0);
        const sysSlider = document.getElementById("system-volume-slider");
        const miniSysSlider = document.getElementById("mini-system-volume-slider");
        const overlaySysSlider = document.getElementById("overlay-system-volume-slider");
        const sysValSpan = document.getElementById("system-volume-val");
        const overlaySysLabel = document.getElementById("overlay-system-volume-label");

        if (sysSlider && document.activeElement !== sysSlider) {
            sysSlider.value = sysVal;
            sysSlider.style.setProperty("--sys-progress", `${sysVal}%`);
        }
        if (miniSysSlider && document.activeElement !== miniSysSlider) {
            miniSysSlider.value = sysVal;
            miniSysSlider.style.setProperty("--sys-progress", `${sysVal}%`);
        }
        if (overlaySysSlider && document.activeElement !== overlaySysSlider) {
            overlaySysSlider.value = sysVal;
            overlaySysSlider.style.setProperty("--sys-progress", `${sysVal}%`);
        }
        if (sysValSpan) sysValSpan.textContent = `${sysVal}%`;
        if (overlaySysLabel) overlaySysLabel.textContent = `${sysVal}%`;
    }

    // 4. Track change detection (auto-advance or remote selection)
    if (status.track_id && Number(status.track_id) !== Number(state.activeTrackId)) {
        selectTrack(status.track_id, false);
    }

    // 5. Native format quality badge & Now Playing banner pills
    updateNativeFormatBadge(status);
    updateAudioQualityPillBanner(status);
}

function updateAudioQualityPillBanner(status = null) {
    const specsPills = [document.getElementById("aq-pill-specs"), document.getElementById("mini-aq-pill-specs"), document.getElementById("overlay-aq-pill-specs")];
    const formatPills = [document.getElementById("aq-pill-format"), document.getElementById("mini-aq-pill-format"), document.getElementById("overlay-aq-pill-format")];
    const modePills = [document.getElementById("aq-pill-mode"), document.getElementById("mini-aq-pill-mode"), document.getElementById("overlay-aq-pill-mode")];

    const track = state.activeTrackId ? trackDetailsCache[state.activeTrackId] : null;

    // 1. Calculate Sample Rate (Hz)
    let sampleRateHz = 44100;
    if (status && status.original_sr && status.original_sr > 0) {
        sampleRateHz = status.original_sr;
    } else if (track && (track.sample_rate || track.samplerate)) {
        sampleRateHz = Number(track.sample_rate || track.samplerate);
    } else if (status && status.native_hw_sr && status.native_hw_sr > 0) {
        sampleRateHz = status.native_hw_sr;
    }

    const srKhz = (sampleRateHz / 1000).toFixed(1);

    // 2. Calculate Bit Depth
    let bits = 16;
    if (status && status.original_bits && Number(status.original_bits) > 0) {
        bits = Number(status.original_bits);
    } else if (track && (track.bit_depth || track.bits_per_sample)) {
        bits = Number(track.bit_depth || track.bits_per_sample);
    } else if (track) {
        const path = (track.file_path || "").toLowerCase();
        if (path.includes("24bit") || path.includes("24-bit") || path.includes("hi-res") || path.includes("hires") || path.includes("pcm_24") || path.includes("24_96") || path.includes("24_192")) {
            bits = 24;
        } else if (path.includes("32bit") || path.includes("32-bit")) {
            bits = 32;
        } else if (path.endsWith(".flac")) {
            if (sampleRateHz > 48000) bits = 24;
            else bits = 16;
        }
    } else if (status && status.native_hw_bits) {
        bits = Number(status.native_hw_bits);
    }

    specsPills.forEach(pill => {
        if (pill) {
            pill.innerHTML = `<i class="fa-solid fa-microchip" style="font-size: 9px; margin-right: 2px;"></i>${srKhz} kHz / ${bits}-bit`;
        }
    });

    // Update main format badge (#player-format-badge)
    updatePlayerFormatBadge(bits, sampleRateHz);

    // 3. Calculate File Format (FLAC, WAV, MP3, AAC, etc.)
    let fmt = "FLAC";
    if (track) {
        const path = (track.file_path || "").toLowerCase();
        if (path.endsWith(".mp3")) fmt = "MP3";
        else if (path.endsWith(".wav")) fmt = "WAV";
        else if (path.endsWith(".m4a") || path.endsWith(".aac")) fmt = "AAC";
        else if (path.endsWith(".ogg")) fmt = "OGG";
    }

    formatPills.forEach(pill => {
        if (pill) {
            pill.innerHTML = `<i class="fa-solid fa-wave-square" style="font-size: 9px;"></i> ${fmt}`;
        }
    });

    // 4. Calculate WASAPI / Audio Output Mode (WASAPI Exclusive vs Shared Mode)
    const isExclusive = (status && (status.wasapi_exclusive === true || status.wasapi_exclusive === "true")) ||
                        (state.pref && (state.pref["dsp-wasapi_exclusive"] === true || state.pref["dsp-wasapi_exclusive"] === "true" || state.pref["dsp-wasapi_exclusive"] !== "false")) ||
                        (window.serverState?.preferences?.["dsp-wasapi_exclusive"] !== false && window.serverState?.preferences?.["dsp-wasapi_exclusive"] !== "false");

    modePills.forEach(pill => {
        if (pill) {
            if (isExclusive) {
                pill.innerHTML = `<i class="fa-solid fa-bolt" style="font-size: 9px;"></i> WASAPI Exclusive`;
                pill.style.background = "rgba(56, 189, 248, 0.18)";
                pill.style.borderColor = "rgba(56, 189, 248, 0.4)";
                pill.style.color = "#38bdf8";
            } else {
                pill.innerHTML = `<i class="fa-solid fa-volume-high" style="font-size: 9px;"></i> Shared Mode`;
                pill.style.background = "rgba(148, 163, 184, 0.15)";
                pill.style.borderColor = "rgba(148, 163, 184, 0.3)";
                pill.style.color = "#94a3b8";
            }
        }
    });
}

let lastAlertError = "";

/**
 * Shows / hides the native audio quality badge in the player bar.
 *   - Green "✓ Bit-Perfect" when exclusive & playing natively
 *   - Red "🚫 Playback Stopped" + Alert when native sample rate is unsupported by hardware
 *   - Hidden in shared mode
 */
function updateNativeFormatBadge(status) {
    let badge = document.getElementById("native-format-badge");

    // Lazily create the badge element once
    if (!badge) {
        badge = document.createElement("div");
        badge.id = "native-format-badge";
        badge.style.cssText = [
            "position:fixed",
            "bottom:88px",          // just above the player bar
            "right:20px",
            "padding:5px 12px",
            "border-radius:20px",
            "font-size:11px",
            "font-weight:700",
            "letter-spacing:0.5px",
            "pointer-events:none",
            "z-index:9990",
            "transition:opacity 0.3s ease, transform 0.3s ease",
            "box-shadow:0 4px 16px rgba(0,0,0,0.4)",
            "display:none",
        ].join(";");
        document.body.appendChild(badge);
    }

    const isExclusive = status.is_exclusive;
    const playbackError = status.playback_error;
    const hwSr = status.native_hw_sr ? `${(status.native_hw_sr / 1000).toFixed(1)}kHz` : "";
    const hwBits = status.native_hw_bits ? `${status.native_hw_bits}bit` : "";

    if (playbackError) {
        // Trigger alert banner once per error
        if (lastAlertError !== playbackError) {
            lastAlertError = playbackError;
            if (typeof showDebugError === "function") {
                showDebugError(`🚫 Native Audio Error: ${playbackError}`);
            }
        }
        // Red Error Badge
        badge.style.display = "block";
        badge.style.background = "linear-gradient(135deg, rgba(239,68,68,0.95), rgba(185,28,28,0.95))";
        badge.style.color = "#fff";
        badge.style.border = "1px solid rgba(239,68,68,0.6)";
        badge.title = playbackError;
        badge.innerHTML = `🚫&nbsp;&nbsp;Native Unsupported · Playback Stopped`;
        return;
    }

    if (!isExclusive || !status.is_playing) {
        badge.style.display = "none";
        return;
    }

    // Green — true bit-perfect
    badge.style.display = "block";
    badge.style.background = "linear-gradient(135deg, rgba(34,197,94,0.95), rgba(16,185,129,0.95))";
    badge.style.color = "#fff";
    badge.style.border = "1px solid rgba(34,197,94,0.5)";
    badge.title = `Bit-Perfect Native: ${hwSr} / ${hwBits} — 0 dB pass-through`;
    badge.innerHTML = `✓&nbsp;&nbsp;Bit-Perfect · ${hwSr} / ${hwBits}`;
}

function startPlayerStatusPolling() {
    if (isWsConnected) return; // Skip polling if WebSocket is active
    if (statusPollInterval) clearInterval(statusPollInterval);
    statusPollInterval = setInterval(async () => {
        try {
            const res = await fetch("/api/player/status");
            if (!res.ok) return;
            const status = await res.json();
            await processPlayerStatus(status);
        } catch (e) { }
    }, 500);
}

function stopPlayerStatusPolling() {
    if (statusPollInterval) {
        clearInterval(statusPollInterval);
        statusPollInterval = null;
    }
}

function connectWebSocket() {
    const wsProtocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const wsUrl = `${wsProtocol}//${window.location.host}/ws`;

    console.log("[WS] Connecting to WebSocket at", wsUrl);
    ws = new WebSocket(wsUrl);

    ws.onopen = () => {
        console.log("[WS] Connected to Sonar WebSocket Server!");
        isWsConnected = true;
        stopPlayerStatusPolling();
    };

    ws.onmessage = async (event) => {
        try {
            const msg = JSON.parse(event.data);
            if (msg.type === "status") {
                await processPlayerStatus(msg.data);
            } else if (msg.type === "queue") {
                // Server pushed a full queue update — no HTTP fetch needed
                const d = msg.data;
                if (Array.isArray(d.queue)) {
                    state.activePlaylist = d.queue;
                    state.queueVersion = d.queue_version ?? state.queueVersion;
                    generateShuffleIndices();
                    updateQueueWidget();
                }
            } else if (msg.type === "favorites") {
                // Server pushed updated favorites and history
                const d = msg.data;
                if (Array.isArray(d.favorites)) {
                    state.favorites = d.favorites;
                }
                if (Array.isArray(d.history)) {
                    state.history = d.history;
                }
                if (state.activeWorkspace === "workspace-favorites" && typeof loadFavoritesWorkspace === "function") {
                    loadFavoritesWorkspace();
                }
            }
        } catch (e) {
            console.error("[WS] Error processing message:", e);
        }
    };

    ws.onclose = () => {
        console.log("[WS] Connection lost. Falling back to HTTP polling. Reconnecting in 3s...");
        isWsConnected = false;
        ws = null;
        startPlayerStatusPolling();
        setTimeout(connectWebSocket, 3000);
    };

    ws.onerror = (err) => {
        console.error("[WS] WebSocket Error: ", err);
    };
}

// Start WebSocket connection routine
connectWebSocket();
function playAudio() {
    fetch("/api/player/resume", { method: "POST" });
}

function pauseAudio() {
    fetch("/api/player/pause", { method: "POST" });
}

function resetPlayerUI() {
    audioTimeCurrent.textContent = "00:00";
    setAudioSliderProgress(0);
}

function syncHeatmapHighlight(currentSec, totalSec) {
    if (totalSec > 0) {
        const activeTrack = state.activeTrackId ? trackDetailsCache[state.activeTrackId] : null;
        if (activeTrack && activeTrack.instrument_presence_timeline && !activeTrack._parsedTimeline) {
            try {
                const pt = activeTrack.instrument_presence_timeline;
                activeTrack._parsedTimeline = typeof pt === "string" ? JSON.parse(pt) : pt;
            } catch(e) {}
        }
        const parsedTimeline = activeTrack ? activeTrack._parsedTimeline : null;
        let activeChord = null;

        if (parsedTimeline && parsedTimeline.length > 0) {
            const actualTimelineLength = parsedTimeline.length;
            const frameIdx = Math.min(actualTimelineLength - 1, Math.floor((currentSec / totalSec) * actualTimelineLength));
            const activeFrame = parsedTimeline[frameIdx];
            if (activeFrame) {
                activeChord = activeFrame.chord;
            }
        }

        // Update live chord badge only when changed
        const chordBadge = document.getElementById("live-chord-badge");
        const chordNameEl = document.getElementById("live-chord-name");
        if (chordBadge) {
            const displayVal = (activeChord && activeChord !== "Unknown" && activeChord !== "None") ? activeChord : "";
            if (chordBadge.dataset.lastChord !== displayVal) {
                chordBadge.dataset.lastChord = displayVal;
                if (displayVal) {
                    if (chordNameEl) {
                        chordNameEl.textContent = displayVal;
                    } else {
                        chordBadge.innerHTML = `<i class="fa-solid fa-music" style="font-size: 9px; margin-right: 4px;"></i>Chord: ${displayVal}`;
                    }
                    chordBadge.style.display = "inline-flex";
                    chordBadge.style.alignItems = "center";
                } else {
                    chordBadge.style.display = "none";
                }
            }
        }

        const totalBlocks = 60;
        const activeBlockIdx = Math.floor((currentSec / totalSec) * totalBlocks);
        if (state.currentActiveBlockIdx !== activeBlockIdx) {
            state.currentActiveBlockIdx = activeBlockIdx;
            const cells = document.querySelectorAll(".heatmap-cell");
            for (let i = 0; i < cells.length; i++) {
                const blockIdx = parseInt(cells[i].getAttribute("data-block-index"));
                if (blockIdx === activeBlockIdx) {
                    cells[i].classList.add("active-block");
                } else {
                    cells[i].classList.remove("active-block");
                }
            }
        }
    } else {
        if (state.currentActiveBlockIdx !== -1) {
            state.currentActiveBlockIdx = -1;
            document.querySelectorAll(".heatmap-cell").forEach(cell => {
                cell.classList.remove("active-block");
            });
        }
    }
}

let isProgressAnimRunning = false;
let lastProgressMetadataSync = 0;
let lastRenderedProgress = -1;

function startProgressAnimation() {
    if (!isProgressAnimRunning && state.isPlaying && !document.hidden) {
        isProgressAnimRunning = true;
        if (!state.basePerfTime) {
            state.baseCurrentTimeSec = state.localPlayTimeSec || 0;
            state.basePerfTime = performance.now();
        }
        requestAnimationFrame(animateProgressSlider);
    }
}

function animateProgressSlider() {
    if (!state.isPlaying || document.hidden || (audioSlider && audioSlider.dataset.dragging)) {
        isProgressAnimRunning = false;
        return; // Pause frame requests when not playing or hidden to save GPU/CPU power
    }

    const now = performance.now();
    let currentSec = 0;
    const msSinceSeek = state.lastSeekTimestamp ? (Date.now() - state.lastSeekTimestamp) : Infinity;
    const inSeekLockOut = msSinceSeek < 1500;

    // Direct browser HTML5 audio element binding when playing in web player mode
    // During seek lock-out: use pinned monotonic clock so the bar stays at seeked position
    if (!inSeekLockOut && audio && !audio.paused && audio.duration > 0 && !isNaN(audio.currentTime)) {
        currentSec = audio.currentTime;
    } else {
        // High-precision monotonic sub-millisecond performance.now() clock interpolation for WASAPI backend
        const elapsed = (now - (state.basePerfTime || now)) / 1000;
        currentSec = Math.min(state.duration || 0, (state.baseCurrentTimeSec || 0) + elapsed);
    }
    
    state.localPlayTimeSec = currentSec;
    const totalSec = state.duration || 0;
    const progress = totalSec > 0 ? Math.max(0, Math.min(1, currentSec / totalSec)) : 0;
    
    // Avoid redundant DOM writes when progress has not changed significantly (> 0.00005)
    if (Math.abs(progress - lastRenderedProgress) > 0.00005) {
        lastRenderedProgress = progress;
        const transformStr = `scaleX(${progress})`;

        const fillEl = document.getElementById("audio-slider-fill");
        if (fillEl) fillEl.style.transform = transformStr;

        const miniProgressFill = document.getElementById("mini-progress-bar-fill");
        if (miniProgressFill) miniProgressFill.style.transform = transformStr;

        if (audioSlider && !audioSlider.dataset.dragging) {
            audioSlider.value = (progress * 100).toFixed(2);
            audioSlider.style.setProperty("--progress", `${(progress * 100).toFixed(2)}%`);
        }
    }

    const formattedCur = formatDuration(currentSec);
    if (audioTimeCurrent && audioTimeCurrent.textContent !== formattedCur) {
        audioTimeCurrent.textContent = formattedCur;
    }
    const miniTimeCurrent = document.getElementById("mini-time-current");
    if (miniTimeCurrent && miniTimeCurrent.textContent !== formattedCur) {
        miniTimeCurrent.textContent = formattedCur;
    }

    if (now - lastProgressMetadataSync >= 100) {
        lastProgressMetadataSync = now;
        updateLyricsSync(currentSec);
        updateActiveSectionIndicator(currentSec);
        syncHeatmapHighlight(currentSec, totalSec);
    }

    requestAnimationFrame(animateProgressSlider);
}



document.addEventListener("visibilitychange", () => {
    if (!document.hidden && state.isPlaying) {
        startProgressAnimation();
    }
});

function updateVolumeIcon(vol) {
    const icon = document.getElementById("audio-volume-icon") || (typeof volumeIcon !== 'undefined' ? volumeIcon : null);
    if (!icon) return;
    if (vol <= 0) icon.className = "fa-solid fa-volume-xmark volume-icon";
    else if (vol < 0.4) icon.className = "fa-solid fa-volume-low volume-icon";
    else icon.className = "fa-solid fa-volume-high volume-icon";
}
// Bind Global UI event listeners
function setupEventListeners() {
    // View mode toggles
    const viewToggles = document.querySelectorAll(".view-toggle");
    viewToggles.forEach(btn => {
        btn.addEventListener("click", () => {
            viewToggles.forEach(b => b.classList.remove("active"));
            btn.classList.add("active");
            libraryViewMode = btn.getAttribute("data-view");
            saveServerState("player-library-view", libraryViewMode);
            renderTracks(state.tracks);
        });
    });

    // Search inputs
    if (searchInput) {
        searchInput.addEventListener("input", () => {
            if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
            searchDebounceTimer = setTimeout(() => {
                state.searchQuery = searchInput.value;
                state.currentPage = 1;
                loadTracks();
                saveServerState("player-search-query", state.searchQuery);
                updateSearchClearBtnVisibility();
                updateAdvancedFiltersBadge();
            }, 30000000000000000); // disable auto-refresh on typing; refresh only on clear or press enter
            updateSearchClearBtnVisibility();
            updateAdvancedFiltersBadge();
        });

        searchInput.addEventListener("keypress", (e) => {
            if (e.key === "Enter") {
                state.searchQuery = searchInput.value;
                state.currentPage = 1;
                pushNavHistoryState({
                    type: "library_search",
                    query: state.searchQuery,
                    page: 1,
                    groupingField: state.activeGroupingField || null,
                    groupingValue: state.activeGroupingValue || null
                });
                loadTracks();
                saveServerState("player-search-query", state.searchQuery);
                updateSearchClearBtnVisibility();
                updateAdvancedFiltersBadge();
            }
        });
    }

    if (searchClearBtn) {
        searchClearBtn.addEventListener("click", () => {
            searchInput.value = "";
            state.searchQuery = "";
            state.currentPage = 1;
            pushNavHistoryState({
                type: "library_search",
                query: "",
                page: 1,
                groupingField: state.activeGroupingField || null,
                groupingValue: state.activeGroupingValue || null
            });
            loadTracks();
            saveServerState("player-search-query", "");
            updateSearchClearBtnVisibility();
            updateAdvancedFiltersBadge();
        });
    }

    // Vocal filters chips
    filterTags.forEach(tag => {
        tag.addEventListener("click", () => {
            filterTags.forEach(t => t.classList.remove("active"));
            tag.classList.add("active");
            state.vocalFilter = tag.getAttribute("data-vocal");
            state.currentPage = 1;
            loadTracks();
            saveServerState("player-filter-vocalFilter", state.vocalFilter);
            updateAdvancedFiltersBadge();
        });
    });

    // Advanced filter dropdowns binding & saving
    document.querySelectorAll("[id^='filter-']").forEach(select => {
        select.addEventListener("change", () => {
            const stateProp = select.getAttribute("data-state");
            if (stateProp) {
                state[stateProp] = select.value;
                state.currentPage = 1;
                loadTracks();
                updateAdvancedFiltersBadge();
                saveServerState("player-filter-" + stateProp, select.value);
            }
        });
    });

    // Advanced filters toggle button
    const btnToggleAdvancedFilters = document.getElementById("btn-toggle-advanced-filters");
    const advancedFiltersPanel = document.getElementById("advanced-filters-panel");
    if (btnToggleAdvancedFilters && advancedFiltersPanel) {
        btnToggleAdvancedFilters.addEventListener("click", () => {
            advancedFiltersPanel.classList.toggle("active");
        });
    }

    // Clear Filters Button event listener
    if (btnClearFilters) {
        btnClearFilters.addEventListener("click", () => {
            // Clear search query
            if (searchInput) searchInput.value = "";
            state.searchQuery = "";
            saveServerState("player-search-query", "");
            updateSearchClearBtnVisibility();

            // Reset vocal filter tags
            state.vocalFilter = "";
            filterTags.forEach(t => {
                if (t.getAttribute("data-vocal") === "") t.classList.add("active");
                else t.classList.remove("active");
            });
            saveServerState("player-filter-vocalFilter", "");

            // Reset character filter
            state.characterFilter = "";
            const fChar = document.getElementById("filter-character");
            if (fChar) fChar.value = "";
            saveServerState("player-filter-characterFilter", "");

            // Reset key filter
            state.keyFilter = "";
            const fKey = document.getElementById("filter-musical-key");
            if (fKey) fKey.value = "";
            saveServerState("player-filter-keyFilter", "");

            // Reset scale filter
            state.scaleFilter = "";
            const fScale = document.getElementById("filter-major-minor");
            if (fScale) fScale.value = "";
            saveServerState("player-filter-scaleFilter", "");

            // Reset emotion filter
            state.emotionFilter = "";
            const fEmotion = document.getElementById("filter-emotion");
            if (fEmotion) fEmotion.value = "";
            saveServerState("player-filter-emotionFilter", "");

            // Reset advanced filters
            const advancedFilters = [
                'strings', 'piano', 'drums', 'complexity', 'choir', 'guitar', 'bass',
                'winds', 'synth', 'brass',
                'dreaminess', 'epicness', 'cinematicness', 'electronicness', 'nostalgia', 'bpm'
            ];
            advancedFilters.forEach(f => {
                state[f + 'Filter'] = "";
                const el = document.getElementById("filter-" + f);
                if (el) el.value = "";
                saveServerState("player-filter-" + f + "Filter", "");
            });

            // Hide advanced filters panel
            const advancedPanel = document.getElementById("advanced-filters-panel");
            if (advancedPanel) advancedPanel.classList.remove("active");

            // Reload tracks
            state.currentPage = 1;
            loadTracks();
            updateAdvancedFiltersBadge();
        });
    }

    function updateAdvancedFiltersBadge() {
        const advancedFilters = [
            state.scaleFilter, state.stringsFilter, state.keyboardsFilter, state.pianoFilter, state.drumsFilter,
            state.complexityFilter, state.choirFilter, state.guitarFilter, state.bassFilter,
            state.windsFilter, state.synthFilter, state.brassFilter,
            state.dreaminessFilter, state.epicnessFilter, state.cinematicnessFilter,
            state.electronicnessFilter, state.nostalgiaFilter, state.bpmFilter
        ];
        const activeCount = advancedFilters.filter(val => val !== "").length;
        const badge = document.getElementById("advanced-filter-count");
        if (badge) {
            badge.textContent = activeCount;
            badge.style.display = activeCount > 0 ? "inline-flex" : "none";
        }
        if (btnToggleAdvancedFilters) {
            if (activeCount > 0) {
                btnToggleAdvancedFilters.classList.add("filters-active");
            } else {
                btnToggleAdvancedFilters.classList.remove("filters-active");
            }
        }

        // Toggle clear filters button visibility
        const hasAnyFilter = activeCount > 0 || state.vocalFilter !== "" || state.characterFilter !== "" || state.keyFilter !== "" || state.scaleFilter !== "" || state.emotionFilter !== "" || state.searchQuery !== "";
        if (btnClearFilters) {
            btnClearFilters.style.display = hasAnyFilter ? "inline-flex" : "none";
        }
    }

    // Initialize badge status
    updateAdvancedFiltersBadge();

    // Playlist Builder filters wiring
    function updatePbAdvancedFiltersBadge() {
        const pbAdvancedFilters = [
            state.pbEmotionFilter, state.pbStringsFilter, state.pbKeyboardsFilter, state.pbPianoFilter, state.pbDrumsFilter,
            state.pbComplexityFilter, state.pbChoirFilter, state.pbGuitarFilter, state.pbBassFilter,
            state.pbWindsFilter, state.pbSynthFilter, state.pbBrassFilter,
            state.pbDreaminessFilter, state.pbEpicnessFilter, state.pbCinematicnessFilter,
            state.pbElectronicnessFilter, state.pbNostalgiaFilter, state.pbBpmFilter
        ];
        const activeCount = pbAdvancedFilters.filter(val => val !== "").length;
        const badge = document.getElementById("pb-advanced-filter-count");
        if (badge) {
            badge.textContent = activeCount;
            badge.style.display = activeCount > 0 ? "inline-flex" : "none";
        }
        const btnToggle = document.getElementById("pb-btn-toggle-advanced-filters");
        if (btnToggle) {
            if (activeCount > 0) {
                btnToggle.classList.add("filters-active");
            } else {
                btnToggle.classList.remove("filters-active");
            }
        }
    }

    document.querySelectorAll("[id^='pb-filter-']").forEach(select => {
        select.addEventListener("change", () => {
            const stateProp = select.getAttribute("data-state");
            if (stateProp) {
                state[stateProp] = select.value;
                loadPlaylistBuilderTracks();
                updatePbAdvancedFiltersBadge();
                saveServerState("player-filter-" + stateProp, select.value);
            }
        });
    });

    const pbBtnToggleAdvancedFilters = document.getElementById("pb-btn-toggle-advanced-filters");
    const pbAdvancedFiltersPanel = document.getElementById("pb-advanced-filters-panel");
    if (pbBtnToggleAdvancedFilters && pbAdvancedFiltersPanel) {
        pbBtnToggleAdvancedFilters.addEventListener("click", () => {
            pbAdvancedFiltersPanel.classList.toggle("active");
        });
    }

    document.querySelectorAll("[data-pb-vocal]").forEach(tag => {
        tag.addEventListener("click", () => {
            document.querySelectorAll("[data-pb-vocal]").forEach(t => t.classList.remove("active"));
            tag.classList.add("active");
            state.pbVocalFilter = tag.getAttribute("data-pb-vocal");
            loadPlaylistBuilderTracks();
            saveServerState("player-filter-pbVocalFilter", state.pbVocalFilter);
        });
    });

    updatePbAdvancedFiltersBadge();

    window.updateAdvancedFiltersBadge = updateAdvancedFiltersBadge;
    window.updatePbAdvancedFiltersBadge = updatePbAdvancedFiltersBadge;

    // Sorting selectors
    if (sortColumn) {
        sortColumn.addEventListener("change", () => {
            state.sortBy = sortColumn.value;
            loadTracks();
            saveServerState("player-sort-col", state.sortBy);
        });
    }

    if (btnSortOrder) {
        btnSortOrder.addEventListener("click", () => {
            const current = btnSortOrder.getAttribute("data-order");
            const nextOrder = current === "asc" ? "desc" : "asc";
            btnSortOrder.setAttribute("data-order", nextOrder);
            btnSortOrder.innerHTML = nextOrder === "asc"
                ? `<i class="fa-solid fa-arrow-down-a-z"></i>`
                : `<i class="fa-solid fa-arrow-up-z-a"></i>`;
            state.sortOrder = nextOrder;
            loadTracks();
            saveServerState("player-sort-order", state.sortOrder);
        });
    }

    // Pagination buttons
    if (paginationFirst) {
        paginationFirst.addEventListener("click", () => {
            if (state.currentPage !== 1) {
                state.currentPage = 1;
                loadTracks();
            }
        });
    }

    if (paginationPrev) {
        paginationPrev.addEventListener("click", () => {
            if (state.currentPage > 1) {
                state.currentPage--;
                loadTracks();
            }
        });
    }

    if (paginationNext) {
        paginationNext.addEventListener("click", () => {
            if (state.currentPage < state.totalPages) {
                state.currentPage++;
                loadTracks();
            }
        });
    }

    if (paginationLast) {
        paginationLast.addEventListener("click", () => {
            if (state.currentPage !== state.totalPages) {
                state.currentPage = state.totalPages;
                loadTracks();
            }
        });
    }

    // Drawer buttons
    if (btnCloseDrawer) {
        btnCloseDrawer.addEventListener("click", () => {
            const container = document.querySelector(".app-container");
            container.classList.add("player-collapsed");
            container.classList.remove("drawer-open");
            if (detailsDrawer) detailsDrawer.classList.remove("drawer-open");
            const btnTogglePlayer = document.getElementById("btn-toggle-player");
            if (btnTogglePlayer) btnTogglePlayer.classList.add("collapsed-active");
        });
    }

    // Drawer tab switcher
    const tabBtns = document.querySelectorAll(".tab-btn");
    const tabContents = document.querySelectorAll(".tab-content");

    tabBtns.forEach(btn => {
        btn.addEventListener("click", () => {
            tabBtns.forEach(t => t.classList.remove("active"));
            tabContents.forEach(c => c.classList.remove("active"));

            btn.classList.add("active");
            const tabId = btn.getAttribute("data-tab");
            const targetContent = document.getElementById(tabId);
            if (targetContent) targetContent.classList.add("active");
        });
    });

    // Library Data Processing Scripts
    const btnTranslateMeta = document.getElementById("settings-btn-translate-meta");
    const btnSyncLyrics = document.getElementById("settings-btn-sync-lyrics");
    const processingStatusMsg = document.getElementById("processing-status-message");
    const consoleLogs = document.getElementById("processing-logs-console");
    const btnClearLogs = document.getElementById("clear-processing-logs");

    let logPollingInterval = null;

    if (btnClearLogs && consoleLogs) {
        btnClearLogs.addEventListener("click", () => {
            consoleLogs.textContent = "";
        });
    }

    const btnCopyLogs = document.getElementById("copy-processing-logs");
    if (btnCopyLogs && consoleLogs) {
        btnCopyLogs.addEventListener("click", () => {
            const text = consoleLogs.textContent || "";
            if (!text.trim()) return;
            navigator.clipboard.writeText(text).then(() => {
                const orig = btnCopyLogs.innerHTML;
                btnCopyLogs.innerHTML = `<i class="fa-solid fa-check"></i> Copied!`;
                setTimeout(() => { btnCopyLogs.innerHTML = orig; }, 2000);
            }).catch(err => {
                console.error("Copy failed:", err);
            });
        });
    }

    const btnStopScript = document.getElementById("stop-processing-script");
    if (btnStopScript) {
        btnStopScript.addEventListener("click", async () => {
            if (processingStatusMsg) processingStatusMsg.textContent = "Stopping all processes...";
            try {
                const res = await fetch("/api/admin/kill-script", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ script: "all" })
                });
                const data = await res.json();
                if (processingStatusMsg) processingStatusMsg.textContent = "🛑 Stopped all background processes.";
                if (consoleLogs) consoleLogs.textContent += "\n[SYSTEM] Stopped all background processes.\n";
            } catch (err) {
                console.error("Error stopping script:", err);
                if (processingStatusMsg) processingStatusMsg.textContent = "Error stopping processes.";
            }
        });
    }

    let currentLogScriptKey = null;

    const startLogPolling = (scriptKey = null) => {
        if (scriptKey) currentLogScriptKey = scriptKey;
        if (logPollingInterval) clearInterval(logPollingInterval);
        let stemCompleteNotified = false;

        logPollingInterval = setInterval(async () => {
            if (!consoleLogs) return;
            try {
                const url = currentLogScriptKey ? `/api/admin/script-logs?key=${currentLogScriptKey}` : "/api/admin/script-logs";
                const res = await fetch(url);
                if (res.ok) {
                    const data = await res.json();
                    if (data.logs !== undefined) {
                        const atBottom = consoleLogs.scrollHeight - consoleLogs.clientHeight <= consoleLogs.scrollTop + 20;
                        consoleLogs.textContent = data.logs;
                        if (atBottom) {
                            consoleLogs.scrollTop = consoleLogs.scrollHeight;
                        }
                        // Auto reload thresholds when Demucs analysis completes
                        if (!stemCompleteNotified && data.logs.includes("STEM ANALYSIS COMPLETE")) {
                            stemCompleteNotified = true;
                            try {
                                await fetch("/api/admin/reload-thresholds", { method: "POST" });
                                if (processingStatusMsg) {
                                    processingStatusMsg.textContent = "✅ Stem analysis done. Filter thresholds updated automatically.";
                                }
                            } catch (_) { }
                        }
                    }
                }
            } catch (err) {
                console.error("Error fetching logs:", err);
            }
        }, 1000);
    };

    const runBackendScript = async (btn, scriptName, btnText) => {
        if (!btn) return;
        btn.disabled = true;
        const originalHtml = btn.innerHTML;
        btn.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Running...`;
        if (processingStatusMsg) processingStatusMsg.textContent = "Task sent to background. Please wait...";
        if (consoleLogs) consoleLogs.textContent = "Initializing script...\n";

        try {
            const res = await fetch("/api/admin/run-script", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ script: scriptName })
            });
            if (res.ok) {
                if (processingStatusMsg) processingStatusMsg.textContent = `${btnText} is running in the background.`;
                startLogPolling(scriptName);
            } else {
                if (processingStatusMsg) processingStatusMsg.textContent = `Failed to start ${btnText}.`;
            }
        } catch (err) {
            console.error("Script error:", err);
            if (processingStatusMsg) processingStatusMsg.textContent = `Error starting ${btnText}.`;
        } finally {
            setTimeout(() => {
                btn.innerHTML = originalHtml;
                btn.disabled = false;
            }, 3000);
        }
    };

    if (btnTranslateMeta) {
        btnTranslateMeta.addEventListener("click", () => runBackendScript(btnTranslateMeta, "translate", "Metadata Translation"));
    }

    if (btnSyncLyrics) {
        btnSyncLyrics.addEventListener("click", () => runBackendScript(btnSyncLyrics, "lyrics_sync", "Whisper Lyrics Sync"));
    }
    const btnTranslateLyrics = document.getElementById("settings-btn-translate-lyrics");
    if (btnTranslateLyrics) {
        btnTranslateLyrics.addEventListener("click", () => runBackendScript(btnTranslateLyrics, "lyrics_trans", "Lyrics Translation"));
    }
    const btnSimilarity = document.getElementById("settings-btn-similarity");
    if (btnSimilarity) {
        btnSimilarity.addEventListener("click", () => runBackendScript(btnSimilarity, "similarity", "Melody Fingerprinting"));
    }
async function loadBackupVaultUI() {
    const tbody = document.getElementById("vault-backups-tbody");
    if (!tbody) return;

    try {
        const res = await fetch("/api/admin/vault/list");
        if (!res.ok) return;
        const data = await res.json();
        const backups = data.backups || [];

        if (backups.length === 0) {
            tbody.innerHTML = `<tr><td colspan="5" style="padding: 24px; text-align: center; color: var(--text-muted);">No backup snapshots found in .backups vault.</td></tr>`;
            return;
        }

        tbody.innerHTML = "";
        backups.forEach(b => {
            const tr = document.createElement("tr");
            tr.style.borderBottom = "1px solid rgba(255,255,255,0.05)";
            
            const isJson = b.filename.endsWith(".json.gz") || b.filename.endsWith(".json");
            const badgeClass = isJson ? "background: rgba(99, 102, 241, 0.2); color: #a5b4fc;" : (b.type === "Auto Change" ? "background: rgba(245, 158, 11, 0.2); color: #fbbf24;" : "background: rgba(16, 185, 129, 0.2); color: #34d399;");
            
            tr.innerHTML = `
                <td style="padding: 12px 16px; font-weight: 600; color: #f8fafc;">
                    <i class="fa-solid ${isJson ? 'fa-file-code' : 'fa-database'}" style="margin-right: 8px; color: ${isJson ? '#818cf8' : '#34d399'};"></i>
                    ${escapeHtml(b.filename)}
                </td>
                <td style="padding: 12px 16px;">
                    <span style="font-size: 11px; padding: 2px 8px; border-radius: 4px; font-weight: 700; ${badgeClass}">
                        ${escapeHtml(b.type)}
                    </span>
                </td>
                <td style="padding: 12px 16px; color: var(--text-muted);">${b.size_mb} MB</td>
                <td style="padding: 12px 16px; color: var(--text-muted);">${b.mtime}</td>
                <td style="padding: 12px 16px; text-align: right;">
                    ${isJson ? '<span style="font-size: 11px; color: var(--text-muted);">Vault Export</span>' : `
                    <button class="btn-restore-snap" data-filename="${escapeHtml(b.filename)}" style="background: rgba(239, 68, 68, 0.15); border: 1px solid rgba(239, 68, 68, 0.3); color: #f87171; border-radius: 6px; padding: 4px 12px; cursor: pointer; font-size: 12px; font-weight: 600; transition: all 0.2s;">
                        <i class="fa-solid fa-rotate-left"></i> Restore
                    </button>`}
                </td>
            `;

            const btnRest = tr.querySelector(".btn-restore-snap");
            if (btnRest) {
                btnRest.addEventListener("click", async () => {
                    const fname = btnRest.getAttribute("data-filename");
                    if (!confirm(`Are you sure you want to restore the database from snapshot "${fname}"?\n\n(A safety backup of your current state will be created automatically before restoring.)`)) return;
                    
                    btnRest.disabled = true;
                    btnRest.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Restoring...`;
                    try {
                        const r = await fetch("/api/admin/vault/restore", {
                            method: "POST",
                            headers: { "Content-Type": "application/json" },
                            body: JSON.stringify({ filename: fname })
                        });
                        const resData = await r.json();
                        if (resData.success) {
                            alert(`Database restored successfully from "${fname}"!`);
                            loadBackupVaultUI();
                            loadTracks();
                        } else {
                            alert(`Restore failed: ${resData.error || 'Unknown error'}`);
                        }
                    } catch (e) {
                        alert(`Restore request error: ${e}`);
                    } finally {
                        btnRest.disabled = false;
                        btnRest.innerHTML = `<i class="fa-solid fa-rotate-left"></i> Restore`;
                    }
                });
            }

            tbody.appendChild(tr);
        });

        // Fetch Flight Data Recorder Audit Logs
        const auditContainer = document.getElementById("vault-audit-logs-container");
        if (auditContainer) {
            try {
                const aRes = await fetch("/api/admin/vault/audit");
                if (aRes.ok) {
                    const aData = await aRes.json();
                    const logs = aData.audit_logs || [];
                    if (logs.length === 0) {
                        auditContainer.innerHTML = `<div style="color: var(--text-muted); text-align: center; padding: 20px;">No audit changesets recorded yet. Native SQLite Flight Recorder triggers active.</div>`;
                    } else {
                        auditContainer.innerHTML = logs.map(l => {
                            const actionColor = l.action === "INSERT" ? "#34d399" : (l.action === "UPDATE" ? "#fbbf24" : "#f87171");
                            const oldVal = l.old_values ? (typeof l.old_values === 'string' ? l.old_values : JSON.stringify(l.old_values)) : "";
                            const newVal = l.new_values ? (typeof l.new_values === 'string' ? l.new_values : JSON.stringify(l.new_values)) : "";
                            return `<div style="margin-bottom: 6px; padding-bottom: 4px; border-bottom: 1px solid rgba(255,255,255,0.05); display: flex; gap: 8px; flex-wrap: nowrap; overflow: hidden;">
                                <span style="color: var(--text-muted); min-width: 130px;">[${escapeHtml(l.timestamp)}]</span>
                                <span style="color: ${actionColor}; font-weight: bold; min-width: 60px;">${escapeHtml(l.action)}</span>
                                <span style="color: #94a3b8; min-width: 90px;">${escapeHtml(l.table_name)} #${escapeHtml(String(l.row_id))}</span>
                                <span style="color: var(--text-muted); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; flex-grow: 1;">${escapeHtml(newVal || oldVal)}</span>
                            </div>`;
                        }).join("");
                    }
                }
            } catch (err) {
                console.error("Audit log error:", err);
            }
        }
    } catch (e) {
        console.error("Failed to load backup vault UI:", e);
    }
}
window.loadBackupVaultUI = loadBackupVaultUI;

    const btnLibraryIndex = document.getElementById("settings-btn-library-index");
    if (btnLibraryIndex) {
        btnLibraryIndex.addEventListener("click", () => runBackendScript(btnLibraryIndex, "library", "Library DB Indexing"));
    }
    const btnIntegrity = document.getElementById("settings-btn-integrity");
    if (btnIntegrity) {
        btnIntegrity.addEventListener("click", () => runBackendScript(btnIntegrity, "integrity", "FLAC Integrity Verification"));
    }
    const btnStemAnalyze = document.getElementById("settings-btn-stem-analyze");
    if (btnStemAnalyze) {
        btnStemAnalyze.addEventListener("click", () => runBackendScript(btnStemAnalyze, "stem_analyze", "Demucs Stem Analysis"));
    }
    const btnClassifyDsp = document.getElementById("settings-btn-classify-dsp");
    if (btnClassifyDsp) {
        btnClassifyDsp.addEventListener("click", () => runBackendScript(btnClassifyDsp, "classify_dsp", "Acoustic & DSP Profiling"));
    }
    const btnBackupVault = document.getElementById("settings-btn-backup-vault");
    if (btnBackupVault) {
        btnBackupVault.addEventListener("click", () => runBackendScript(btnBackupVault, "backup_vault", "Database Backup Vault Snapshot"));
    }

    const btnVaultCreate = document.getElementById("btn-vault-create-db");
    if (btnVaultCreate) {
        btnVaultCreate.addEventListener("click", async () => {
            btnVaultCreate.disabled = true;
            btnVaultCreate.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Creating Snapshot...`;
            try {
                const r = await fetch("/api/admin/vault/create", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ type: "snapshot" }) });
                const d = await r.json();
                if (d.success) {
                    loadBackupVaultUI();
                } else {
                    alert(`Error creating snapshot: ${d.error}`);
                }
            } catch (e) {
                alert(`Error: ${e}`);
            } finally {
                btnVaultCreate.disabled = false;
                btnVaultCreate.innerHTML = `<i class="fa-solid fa-plus"></i> Create Snapshot`;
            }
        });
    }

    const btnVaultExport = document.getElementById("btn-vault-export-json");
    if (btnVaultExport) {
        btnVaultExport.addEventListener("click", async () => {
            btnVaultExport.disabled = true;
            btnVaultExport.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Exporting JSON...`;
            try {
                const r = await fetch("/api/admin/vault/create", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ type: "export" }) });
                const d = await r.json();
                if (d.success) {
                    loadBackupVaultUI();
                } else {
                    alert(`Error exporting JSON: ${d.error}`);
                }
            } catch (e) {
                alert(`Error: ${e}`);
            } finally {
                btnVaultExport.disabled = false;
                btnVaultExport.innerHTML = `<i class="fa-solid fa-file-export"></i> Export AI Feature Vault (JSON)`;
            }
        });
    }

    const btnVaultRefresh = document.getElementById("btn-vault-refresh");
    if (btnVaultRefresh) {
        btnVaultRefresh.addEventListener("click", () => loadBackupVaultUI());
    }

    const btnVaultUndo = document.getElementById("btn-vault-undo-last");
    if (btnVaultUndo) {
        btnVaultUndo.addEventListener("click", async () => {
            if (!confirm("Are you sure you want to revert the last database change recorded in Flight Data Recorder?")) return;
            btnVaultUndo.disabled = true;
            btnVaultUndo.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Undoing...`;
            try {
                const r = await fetch("/api/admin/vault/undo", { method: "POST" });
                const d = await r.json();
                if (d.success) {
                    alert(d.message);
                    loadBackupVaultUI();
                    loadTracks();
                } else {
                    alert(`Undo failed: ${d.error}`);
                }
            } catch (e) {
                alert(`Error: ${e}`);
            } finally {
                btnVaultUndo.disabled = false;
                btnVaultUndo.innerHTML = `<i class="fa-solid fa-rotate-left"></i> Undo Last DB Change`;
            }
        });
    }

    const btnToggleVault = document.getElementById("btn-toggle-vault-panel");
    const vaultContainer = document.getElementById("vault-panel-container");
    if (btnToggleVault && vaultContainer) {
        btnToggleVault.addEventListener("click", () => {
            const isHidden = vaultContainer.style.display === "none";
            if (isHidden) {
                vaultContainer.style.display = "block";
                btnToggleVault.innerHTML = `<i class="fa-solid fa-eye-slash"></i> Hide Backup Vault`;
                loadBackupVaultUI();
            } else {
                vaultContainer.style.display = "none";
                btnToggleVault.innerHTML = `<i class="fa-solid fa-folder-open"></i> Show Backup Vault`;
            }
        });
    }

    // Also poll logs when page loads if settings panel is open
    startLogPolling();


    // Settings panel scan operations
    const settingsRescan = document.getElementById("settings-btn-rescan");
    const togglePreTranslate = document.getElementById("settings-toggle-pre-translate");

    // Initialize toggle state from localStorage
    if (togglePreTranslate) {
        const savedPreTranslate = localStorage.getItem("sonar-auto-translate-pre-scan");
        togglePreTranslate.checked = savedPreTranslate === "true";
        togglePreTranslate.addEventListener("change", () => {
            localStorage.setItem("sonar-auto-translate-pre-scan", togglePreTranslate.checked);
        });
    }

    if (settingsRescan) {
        settingsRescan.addEventListener("click", async () => {
            const autoTranslate = togglePreTranslate ? togglePreTranslate.checked : false;

            settingsRescan.disabled = true;
            settingsRescan.innerHTML = autoTranslate
                ? `<i class="fa-solid fa-spinner fa-spin"></i> Translating & Scanning...`
                : `<i class="fa-solid fa-spinner fa-spin"></i> Scanning...`;

            if (autoTranslate && processingStatusMsg) {
                processingStatusMsg.textContent = "Auto-Translation running before Scan. Please watch logs...";
                if (consoleLogs) consoleLogs.textContent = "Initializing pre-scan translation...\n";
                startLogPolling();
            }

            try {
                const url = autoTranslate ? "/api/scan?translate=true" : "/api/scan";
                const res = await fetch(url);
                if (res.ok) {
                    const data = await res.json();
                    alert(autoTranslate
                        ? "Translation & Database Scan Completed Successfully!"
                        : "Database Scan Completed Successfully!"
                    );
                    if (autoTranslate && processingStatusMsg) {
                        processingStatusMsg.textContent = "Pre-scan translation and database update completed.";
                    }
                    loadStats();
                    loadTracks();
                } else {
                    alert("Scanning operation failed.");
                    if (autoTranslate && processingStatusMsg) {
                        processingStatusMsg.textContent = "Pre-scan translation/scan execution failed.";
                    }
                }
            } catch (err) {
                console.error("Scan error:", err);
                if (autoTranslate && processingStatusMsg) {
                    processingStatusMsg.textContent = "Error executing pre-scan translation/scan.";
                }
            } finally {
                settingsRescan.disabled = false;
                settingsRescan.innerHTML = `<i class="fa-solid fa-rotate"></i> Rescan Library`;
            }
        });
    }

    // Reset cache listener
    const settingsResetCache = document.getElementById("settings-btn-reset-cache");
    if (settingsResetCache) {
        settingsResetCache.addEventListener("click", () => {

            alert("Local State Cache Purged. Refreshing UI...");
            window.location.reload();
        });
    }

    // Reload Backend listener
    const settingsReloadBackend = document.getElementById("settings-btn-reload");
    if (settingsReloadBackend) {
        settingsReloadBackend.addEventListener("click", async () => {
            if (confirm("Are you sure you want to restart the application backend server?")) {
                settingsReloadBackend.disabled = true;
                settingsReloadBackend.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Restarting...`;
                try {
                    await flushServerState();
                    await fetch("/api/reload");
                } catch (err) { }
                setTimeout(() => {
                    window.location.reload();
                }, 1500);
            }
        });
    }

    // Exit App listener
    const settingsExitApp = document.getElementById("settings-btn-exit");
    if (settingsExitApp) {
        settingsExitApp.addEventListener("click", async () => {
            if (confirm("Are you sure you want to close the application? The server will shutdown.")) {
                settingsExitApp.disabled = true;
                settingsExitApp.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Exiting...`;
                try {
                    await flushServerState();
                    await fetch("/api/shutdown");
                } catch (err) { }
                document.body.innerHTML = `
                    <div style="display:flex; flex-direction:column; align-items:center; justify-content:center; height:100vh; color:#f87171; font-family:sans-serif; background:#090710; text-align:center; padding: 20px;">
                        <i class="fa-solid fa-power-off" style="font-size: 48px; margin-bottom: 16px; color:#ef4444;"></i>
                        <h2>Application backend has been shut down cleanly.</h2>
                        <p style="color:#9ca3af; margin-top:8px;">You can now safely close this window.</p>
                    </div>
                `;
            }
        });
    }

    // Sidebar collapse toggler
    document.querySelectorAll(".btn-toggle-sidebar").forEach(btn => {
        btn.addEventListener("click", () => {
            document.querySelector(".app-container").classList.toggle("sidebar-collapsed");
        });
    });

    // Auto-close sidebar on mobile when clicking content area
    const appMain = document.querySelector(".app-main");
    if (appMain) {
        appMain.addEventListener("click", () => {
            if (window.innerWidth < 768) {
                const container = document.querySelector(".app-container");
                if (container && !container.classList.contains("sidebar-collapsed")) {
                    container.classList.add("sidebar-collapsed");
                }
            }
        });
    }

    // Player details drawer / full-screen view toggler
    const toggleFullPlayerDrawer = (e) => {
        if (e) e.stopPropagation();
        const fullPlayerView = document.getElementById("full-player-view");
        const miniPlayerWidget = document.getElementById("mini-player-widget");
        const container = document.querySelector(".app-container") || document.getElementById("app-container");
        const detailsDrawer = document.getElementById("details-drawer");

        if (window.innerWidth <= 768 && fullPlayerView) {
            const isActive = fullPlayerView.classList.toggle("active");
            document.body.classList.toggle("full-player-active", isActive);
            if (miniPlayerWidget) {
                if (isActive) {
                    miniPlayerWidget.style.setProperty("display", "none", "important");
                } else {
                    miniPlayerWidget.style.removeProperty("display");
                }
            }
        } else if (container) {
            const isCollapsed = container.classList.toggle("player-collapsed");
            if (isCollapsed) {
                container.classList.remove("drawer-open");
                if (detailsDrawer) detailsDrawer.classList.remove("drawer-open");
            } else {
                container.classList.add("drawer-open");
                if (detailsDrawer) detailsDrawer.classList.add("drawer-open");
            }
        } else if (detailsDrawer) {
            detailsDrawer.classList.toggle("drawer-open");
        }
    };

    document.querySelectorAll(".btn-toggle-player, #btn-toggle-player, .mini-player-info-group, #mini-player-art, #mini-player-title").forEach(el => {
        el.addEventListener("click", toggleFullPlayerDrawer);
    });

    const btnClosePlayer = document.getElementById("btn-close-player");
    if (btnClosePlayer) {
        btnClosePlayer.addEventListener("click", () => {
            const fullPlayerView = document.getElementById("full-player-view");
            const miniPlayerWidget = document.getElementById("mini-player-widget");
            if (fullPlayerView) fullPlayerView.classList.remove("active");
            document.body.classList.remove("full-player-active");
            if (miniPlayerWidget) {
                miniPlayerWidget.style.removeProperty("display");
            }
        });
    }

    // Player layout mode enforcement (Right side layout)
    const container = document.querySelector(".app-container");
    if (container) {
        container.classList.remove("layout-bottom");
    }

    // Bind search input filter for themes list
    const themesSearchInput = document.getElementById("themes-search-input");
    if (themesSearchInput) {
        themesSearchInput.addEventListener("input", () => {
            loadThemeExplorer();
        });
    }

    // Bind Playback Queue workspace controls
    const queueShuffleToggle = document.getElementById("queue-btn-shuffle-toggle");
    if (queueShuffleToggle) {
        queueShuffleToggle.addEventListener("click", () => {
            const btnShuffle = document.getElementById("audio-btn-shuffle");
            if (btnShuffle) {
                btnShuffle.click(); // reuse existing shuffle toggle logic
                loadQueueWorkspace();
            }
        });
    }

    const queueClearBtn = document.getElementById("queue-btn-clear");
    if (queueClearBtn) {
        queueClearBtn.addEventListener("click", () => {
            state.activePlaylist = [];
            state.shuffleIndices = [];
            state.activeTrackId = null;
            if (audio) audio.src = ""; // FIX: Added null check
            setAudioSliderProgress(0);
            audioTimeCurrent.textContent = "00:00";
            audioTimeTotal.textContent = "00:00";
            playPauseBtn.innerHTML = `<i class="fa-solid fa-play"></i>`;
            state.isPlaying = false;

            // Hide details drawer
            const container = document.querySelector(".app-container");
            container.classList.add("player-collapsed");
            container.classList.remove("drawer-open");
            if (detailsDrawer) detailsDrawer.classList.remove("drawer-open");

            loadQueueWorkspace();
            updateQueueWidget();
            alert("Playback queue cleared.");
        });
    }

    // Theme toggle button click handler
    const btnThemeToggle = document.getElementById("btn-theme-toggle");
    if (btnThemeToggle) {
        btnThemeToggle.addEventListener("click", () => {
            const isCurrentLight = document.body.classList.contains("light-theme");
            const nextTheme = isCurrentLight ? "sonar-dark" : "alabaster-light";
            applyTheme(nextTheme);
        });
    }

    // Collapse bottom panel inside details drawer handler
    const btnCollapseBottom = document.getElementById("btn-collapse-bottom");
    if (btnCollapseBottom) {
        btnCollapseBottom.addEventListener("click", () => {
            if (detailsDrawer) {
                const isCollapsed = detailsDrawer.classList.toggle("bottom-collapsed");
                const topSection = document.getElementById("drawer-top-section");
                if (isCollapsed) {
                    if (topSection) {
                        topSection.dataset.savedHeight = topSection.style.height;
                        topSection.style.height = "";
                    }
                } else {
                    if (topSection) {
                        if (topSection.dataset.savedHeight) {
                            topSection.style.height = topSection.dataset.savedHeight;
                        } else {
                            const savedTopHeight = (window.serverState?.preferences?.["player-drawer-top-height"]);
                            if (savedTopHeight) {
                                topSection.style.height = `${savedTopHeight}px`;
                            }
                        }
                    }
                }
                const icon = btnCollapseBottom.querySelector("i");
                if (icon) {
                    if (isCollapsed) {
                        icon.className = "fa-solid fa-chevron-up";
                    } else {
                        icon.className = "fa-solid fa-chevron-down";
                    }
                }
            }
        });
    }
}

// Bind DSP Workstation UI Control Slider/Checkbox elements
function setupDSPHandlers() {
    const updateLabels = () => {
        const preampVal = parseFloat(document.getElementById("dsp-slider-preamp")?.value ?? 0);
        const bassVal = parseInt(document.getElementById("dsp-slider-bass")?.value ?? 0);
        const eqVal = parseInt(document.getElementById("dsp-slider-eq")?.value ?? 0);
        const vocalsVal = parseInt(document.getElementById("dsp-slider-vocals")?.value ?? 0);
        const airVal = parseInt(document.getElementById("dsp-slider-air")?.value ?? 0);
        const warmthVal = parseInt(document.getElementById("dsp-slider-warmth")?.value ?? 40);
        const stereoVal = parseInt(document.getElementById("dsp-slider-stereo")?.value ?? 100);

        const valPreamp = document.getElementById("dsp-val-preamp");
        if (valPreamp) valPreamp.textContent = `${preampVal > 0 ? '+' : ''}${preampVal} dB`;

        const valBass = document.getElementById("dsp-val-bass");
        if (valBass) valBass.textContent = `${bassVal > 0 ? '+' : ''}${bassVal} dB`;

        const valEq = document.getElementById("dsp-val-eq");
        if (valEq) valEq.textContent = `${eqVal > 0 ? '+' : ''}${eqVal} dB`;

        const valVocals = document.getElementById("dsp-val-vocals");
        if (valVocals) valVocals.textContent = `${vocalsVal > 0 ? '+' : ''}${vocalsVal} dB`;

        const valAir = document.getElementById("dsp-val-air");
        if (valAir) valAir.textContent = `${airVal > 0 ? '+' : ''}${airVal} dB`;

        const valWarmth = document.getElementById("dsp-val-warmth");
        if (valWarmth) valWarmth.textContent = `${warmthVal}%`;

        const valStereo = document.getElementById("dsp-val-stereo");
        if (valStereo) valStereo.textContent = `${stereoVal}%`;
    };

    updateLabels();

    let dspThrottleTimer = null;
    const sendDspUpdate = () => {
        const preampVal = parseFloat(document.getElementById("dsp-slider-preamp")?.value ?? 0);
        const bassVal = parseInt(document.getElementById("dsp-slider-bass")?.value ?? 0);
        const eqVal = parseInt(document.getElementById("dsp-slider-eq")?.value ?? 0);
        const vocalsVal = parseInt(document.getElementById("dsp-slider-vocals")?.value ?? 0);
        const airVal = parseInt(document.getElementById("dsp-slider-air")?.value ?? 0);
        const warmthVal = parseInt(document.getElementById("dsp-slider-warmth")?.value ?? 40);
        const stereoVal = parseInt(document.getElementById("dsp-slider-stereo")?.value ?? 100);

        const cbPreamp = document.getElementById("dsp-cb-preamp")?.checked ?? true;
        const cbBass = document.getElementById("dsp-cb-bass")?.checked ?? true;
        const cbEq = document.getElementById("dsp-cb-eq")?.checked ?? true;
        const cbVocals = document.getElementById("dsp-cb-vocals")?.checked ?? true;
        const cbAir = document.getElementById("dsp-cb-air")?.checked ?? true;
        const cbWarmth = document.getElementById("dsp-cb-warmth")?.checked ?? true;
        const cbStereo = document.getElementById("dsp-cb-stereo")?.checked ?? true;
        const cbLimiter = document.getElementById("dsp-cb-limiter")?.checked ?? true;

        const dsp_data = {
            preamp: cbPreamp ? preampVal : 0.0,
            cb_preamp: cbPreamp,
            eq_bass: bassVal,
            eq_mid: eqVal,
            eq_vocals: vocalsVal,
            eq_air: airVal,
            warmth: warmthVal,
            width: stereoVal,

            cb_bass: cbBass,
            cb_mid: cbEq,
            cb_vocals: cbVocals,
            cb_air: cbAir,
            cb_warmth: cbWarmth,
            cb_stereo: cbStereo
        };
        fetch("/api/player/dsp", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(dsp_data)
        });

        // Persist DSP state to server preferences so it stays saved across reloads
        const dspSaveObj = {
            preamp: preampVal,
            cb_preamp: cbPreamp,
            stereo: stereoVal,
            cb_stereo: cbStereo,
            eq: eqVal,
            cb_eq: cbEq,
            bass: bassVal,
            cb_bass: cbBass,
            warmth: warmthVal,
            cb_warmth: cbWarmth,
            vocals: vocalsVal,
            cb_vocals: cbVocals,
            air: airVal,
            cb_air: cbAir,
            cb_limiter: cbLimiter
        };
        saveServerState("dsp-active-state", JSON.stringify(dspSaveObj));
    };

    document.querySelectorAll(".dsp-effect-controls input").forEach(el => {
        el.addEventListener("input", () => {
            updateLabels();
            if (dspThrottleTimer) clearTimeout(dspThrottleTimer);
            dspThrottleTimer = setTimeout(sendDspUpdate, 50);
        });
    });

    // Pre-amp lives in the EQ card rather than .dsp-effect-controls, so bind
    // it explicitly. Without this listener the slider only moved visually.
    const preampSlider = document.getElementById("dsp-slider-preamp");
    if (preampSlider) {
        preampSlider.addEventListener("input", () => {
            updateLabels();
            if (dspThrottleTimer) clearTimeout(dspThrottleTimer);
            dspThrottleTimer = setTimeout(sendDspUpdate, 50);
        });
    }

    document.querySelectorAll("#dsp-effects-list input[type='checkbox']").forEach(el => {
        el.addEventListener("change", () => {
            sendDspUpdate();
        });
    });

    const globalSwitch = document.getElementById("dsp-global-enable");
    if (globalSwitch) {
        globalSwitch.addEventListener("change", () => {
            setDSPBypass(!globalSwitch.checked);
        });
    }

    const controlBtn = document.getElementById("audio-btn-dsp-ab");
    if (controlBtn) {
        controlBtn.addEventListener("click", () => {
            const currentBypass = !controlBtn.classList.contains("dsp-active");
            setDSPBypass(!currentBypass);
        });
    }

    // -------------------------------------------------------------
    // Preset Manager Controls (Select, Save, Delete, Bulk Action)
    // -------------------------------------------------------------
    const presetSelect = document.getElementById("dsp-preset-select");
    const presetNameInput = document.getElementById("dsp-preset-name");
    const btnSavePreset = document.getElementById("dsp-btn-save-preset");
    const btnDeletePreset = document.getElementById("dsp-btn-delete-preset");
    const btnEnableAll = document.getElementById("dsp-btn-enable-all");
    const btnDisableAll = document.getElementById("dsp-btn-disable-all");
    const btnResetDefaults = document.getElementById("dsp-btn-reset");
    const activePresetLabel = document.getElementById("dsp-current-preset");

    const applyPresetMap = (p) => {
        if (!p) return;
        const setVal = (id, checked, val) => {
            const cb = document.getElementById(`dsp-cb-${id}`);
            const sl = document.getElementById(`dsp-slider-${id}`);
            if (cb && checked !== undefined) cb.checked = !!checked;
            if (sl && val !== undefined) sl.value = val;
        };
        setVal("stereo", p.cb_stereo, p.stereo);
        setVal("eq", p.cb_eq, p.eq);
        setVal("bass", p.cb_bass, p.bass);
        setVal("warmth", p.cb_warmth, p.warmth);
        setVal("vocals", p.cb_vocals, p.vocals);
        setVal("air", p.cb_air, p.air);
        const cbLimiter = document.getElementById("dsp-cb-limiter");
        if (cbLimiter && p.cb_limiter !== undefined) cbLimiter.checked = !!p.cb_limiter;

        updateLabels();
        sendDspUpdate();
    };

    if (presetSelect) {
        presetSelect.addEventListener("change", (e) => {
            const pId = e.target.value;
            const presets = loadDSPPresets();
            if (presets[pId]) {
                applyPresetMap(presets[pId]);
                if (activePresetLabel) {
                    activePresetLabel.textContent = presetSelect.options[presetSelect.selectedIndex].text;
                }
            }
        });
    }

    if (btnSavePreset) {
        btnSavePreset.addEventListener("click", () => {
            const rawName = presetNameInput ? presetNameInput.value.trim() : "";
            if (!rawName) {
                alert("Please enter a name for the new preset.");
                return;
            }
            const pId = rawName.toLowerCase().replace(/[^a-z0-9]/g, "_");
            const currentObj = {
                stereo: parseInt(document.getElementById("dsp-slider-stereo")?.value ?? 100),
                cb_stereo: document.getElementById("dsp-cb-stereo")?.checked ?? true,
                eq: parseInt(document.getElementById("dsp-slider-eq")?.value ?? 0),
                cb_eq: document.getElementById("dsp-cb-eq")?.checked ?? true,
                bass: parseInt(document.getElementById("dsp-slider-bass")?.value ?? 0),
                cb_bass: document.getElementById("dsp-cb-bass")?.checked ?? true,
                warmth: parseInt(document.getElementById("dsp-slider-warmth")?.value ?? 40),
                cb_warmth: document.getElementById("dsp-cb-warmth")?.checked ?? false,
                vocals: parseInt(document.getElementById("dsp-slider-vocals")?.value ?? 0),
                cb_vocals: document.getElementById("dsp-cb-vocals")?.checked ?? false,
                air: parseInt(document.getElementById("dsp-slider-air")?.value ?? 0),
                cb_air: document.getElementById("dsp-cb-air")?.checked ?? false,
                cb_limiter: document.getElementById("dsp-cb-limiter")?.checked ?? true
            };

            const presets = loadDSPPresets();
            presets[pId] = currentObj;
            saveServerState("dsp-presets", JSON.stringify(presets));

            if (presetSelect) {
                let existingOpt = Array.from(presetSelect.options).find(o => o.value === pId);
                if (!existingOpt) {
                    existingOpt = document.createElement("option");
                    existingOpt.value = pId;
                    existingOpt.text = rawName;
                    presetSelect.appendChild(existingOpt);
                }
                presetSelect.value = pId;
            }
            if (activePresetLabel) activePresetLabel.textContent = rawName;
            if (presetNameInput) presetNameInput.value = "";
            alert(`Preset "${rawName}" saved successfully!`);
        });
    }

    const btnSetDefault = document.getElementById("dsp-btn-set-default");
    if (btnSetDefault) {
        btnSetDefault.addEventListener("click", () => {
            if (!presetSelect) return;
            const pId = presetSelect.value;
            const presetText = presetSelect.options[presetSelect.selectedIndex]?.text || pId;
            saveServerState("dsp-default-preset-id", pId);
            flushServerState();
            if (activePresetLabel) {
                activePresetLabel.textContent = `${presetText} (Default)`;
            }
            alert(`Preset "${presetText}" is now set as your startup default!`);
        });
    }

    if (btnDeletePreset) {
        btnDeletePreset.addEventListener("click", () => {
            if (!presetSelect) return;
            const pId = presetSelect.value;
            if (["default", "cinematic", "latenight", "animevocals"].includes(pId)) {
                alert("Cannot delete built-in studio presets.");
                return;
            }
            const presets = loadDSPPresets();
            delete presets[pId];
            saveServerState("dsp-presets", JSON.stringify(presets));

            const opt = Array.from(presetSelect.options).find(o => o.value === pId);
            if (opt) opt.remove();
            presetSelect.value = "default";
            presetSelect.dispatchEvent(new Event("change"));
            alert("Custom preset deleted.");
        });
    }

    if (btnEnableAll) {
        btnEnableAll.addEventListener("click", () => {
            document.querySelectorAll("#dsp-effects-list input[type='checkbox']").forEach(cb => cb.checked = true);
            sendDspUpdate();
        });
    }

    if (btnDisableAll) {
        btnDisableAll.addEventListener("click", () => {
            document.querySelectorAll("#dsp-effects-list input[type='checkbox']").forEach(cb => {
                if (cb.id !== "dsp-cb-limiter") cb.checked = false;
            });
            sendDspUpdate();
        });
    }

    if (btnResetDefaults) {
        btnResetDefaults.addEventListener("click", () => {
            if (presetSelect) presetSelect.value = "default";
            const presets = loadDSPPresets();
            if (presets.default) applyPresetMap(presets.default);
            if (activePresetLabel) activePresetLabel.textContent = "Default Flat";
        });
    }
}

// Format Duration
function formatDuration(sec) {
    if (isNaN(sec)) return "00:00";
    const min = Math.floor(sec / 60);
    const remaining = Math.floor(sec % 60);
    return `${min.toString().padStart(2, '0')}:${remaining.toString().padStart(2, '0')}`;
}

function escapeJsParam(str) {
    if (str === null || str === undefined) return "";
    return String(str)
        .replace(/\\/g, '\\\\')
        .replace(/'/g, "\\'")
        .replace(/"/g, '\\"');
}

function renderArtistLinks(artistStr, isClickableInList = true) {
    if (!artistStr) return "";
    const parts = artistStr.split(',').map(s => s.trim()).filter(Boolean);
    if (parts.length === 0) return "";
    
    const cvPattern = /^(.*?)\s*\((?:CV[\.:\s]|CV:\s*)([^\)]+)\)$/i;

    return parts.map(part => {
        const style = isClickableInList
            ? `cursor: pointer; color: var(--accent-cyan); text-decoration: underline; text-underline-offset: 3px; font-weight: 500;`
            : `cursor: pointer; text-decoration: underline;`;
        
        const cvMatch = part.match(cvPattern);
        if (cvMatch) {
            const charName = cvMatch[1].trim();
            const vaName = cvMatch[2].strip ? cvMatch[2].strip() : cvMatch[2].trim();
            return `<span style="${style}" onclick="event.stopPropagation(); filterByArtist(decodeURIComponent('${escapeJsParam(charName)}'))">${escapeHtml(charName)}</span> ` +
                   `<span style="opacity: 0.7; font-size: 0.95em;">(CV: <span style="${style}" onclick="event.stopPropagation(); filterByArtist(decodeURIComponent('${escapeJsParam(vaName)}'))">${escapeHtml(vaName)}</span>)</span>`;
        }
        return `<span style="${style}" onclick="event.stopPropagation(); filterByArtist(decodeURIComponent('${escapeJsParam(part)}'))">${escapeHtml(part)}</span>`;
    }).join(', ');
}

// Syncing active lyrics line
function findActiveLyricIndex(currentTime) {
    const lines = state.lyricLines;
    if (!lines || lines.length === 0) return -1;

    let low = 0;
    let high = lines.length - 1;
    let result = -1;

    while (low <= high) {
        const mid = Math.floor((low + high) / 2);
        if (lines[mid].time <= currentTime) {
            result = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    return result !== -1 ? lines[result].index : -1;
}

let lastActiveLyricIdx = -1;
function updateLyricsSync(currentTime) {
    if (!state.lyricLines || state.lyricLines.length === 0) return;

    const activeIdx = findActiveLyricIndex(currentTime);

    if (activeIdx !== -1 && activeIdx !== lastActiveLyricIdx) {
        // 1. Sidebar details drawer lyrics
        const activeEl = document.getElementById(`lyric-line-${activeIdx}`);
        if (activeEl) {
            if (lastActiveLyricIdx !== -1) {
                const prevActive = document.getElementById(`lyric-line-${lastActiveLyricIdx}`);
                if (prevActive) prevActive.classList.remove("active");
            } else {
                const prevActive = document.querySelector("#lyrics-content .lyrics-line.active");
                if (prevActive) prevActive.classList.remove("active");
            }
            activeEl.classList.add("active");

            const container = document.querySelector(".lyrics-container");
            if (container) {
                const targetScroll = activeEl.offsetTop - (container.clientHeight / 2) + (activeEl.clientHeight / 2);
                container.scrollTo({ top: targetScroll, behavior: "smooth" });
            }
        }

        // 2. Fullscreen presentation overlay lyrics
        const fsActiveEl = document.getElementById(`fs-lyric-line-${activeIdx}`);
        if (fsActiveEl) {
            if (lastActiveLyricIdx !== -1) {
                const prevFsActive = document.getElementById(`fs-lyric-line-${lastActiveLyricIdx}`);
                if (prevFsActive) prevFsActive.classList.remove("active");
            } else {
                const prevFsActive = document.querySelector("#fs-lyrics-content .lyrics-line.active");
                if (prevFsActive) prevFsActive.classList.remove("active");
            }
            fsActiveEl.classList.add("active");

            const fsContainer = document.querySelector(".fs-lyrics-container");
            if (fsContainer) {
                const targetScroll = fsActiveEl.offsetTop - (fsContainer.clientHeight / 2) + (fsActiveEl.clientHeight / 2);
                fsContainer.scrollTo({ top: targetScroll, behavior: "smooth" });
            }
        }

        lastActiveLyricIdx = activeIdx;
    }
}

// Bind click events on lyrics container for seek-on-click
const mainLyricsContent = document.getElementById("lyrics-content");
if (mainLyricsContent) {
    mainLyricsContent.addEventListener("click", (e) => {
        const lineEl = e.target.closest(".lyrics-line");
        if (!lineEl) return;
        const timeVal = lineEl.getAttribute("data-time");
        if (timeVal) {
            seekAudioTo(parseFloat(timeVal));
        }
    });
}

const fsLyricsContent = document.getElementById("fs-lyrics-content");
if (fsLyricsContent) {
    fsLyricsContent.addEventListener("click", (e) => {
        const lineEl = e.target.closest(".lyrics-line");
        if (!lineEl) return;
        const timeVal = lineEl.getAttribute("data-time");
        if (timeVal) {
            seekAudioTo(parseFloat(timeVal));
        }
    });
}

// Fullscreen presenter visualizer toggle
let fsVisualizerAnimFrame = null;

function startFullscreenVisualizerSpectrum() {
    if (fsVisualizerAnimFrame) {
        cancelAnimationFrame(fsVisualizerAnimFrame);
        fsVisualizerAnimFrame = null;
    }

    const canvas = document.getElementById("fs-visualizer-canvas");
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const barCount = 32;
    if (!window._fsPeakCaps || window._fsPeakCaps.length !== barCount) {
        window._fsPeakCaps = new Array(barCount).fill(0);
        window._fsBarHeights = new Array(barCount).fill(0);
    }

    function drawFrame() {
        const fsOverlay = document.getElementById("fullscreen-overlay");
        if (!fsOverlay || (fsOverlay.style.display !== "flex" && !fsOverlay.classList.contains("fs-active"))) {
            if (fsVisualizerAnimFrame) cancelAnimationFrame(fsVisualizerAnimFrame);
            fsVisualizerAnimFrame = null;
            return;
        }

        const rect = canvas.getBoundingClientRect();
        const dpr = window.devicePixelRatio || 1;
        const width = rect.width || canvas.clientWidth || 320;
        const height = rect.height || canvas.clientHeight || 180;

        if (canvas.width !== Math.floor(width * dpr) || canvas.height !== Math.floor(height * dpr)) {
            canvas.width = Math.floor(width * dpr);
            canvas.height = Math.floor(height * dpr);
        }

        ctx.save();
        ctx.scale(dpr, dpr);
        ctx.clearRect(0, 0, width, height);

        const isPlaying = state.isPlaying;
        const padding = 4;
        const totalPadding = (barCount + 1) * padding;
        const barWidth = Math.max(3, (width - totalPadding) / barCount);
        const now = Date.now() / 1000;

        for (let i = 0; i < barCount; i++) {
            let targetHeightPct = 0;
            if (isPlaying) {
                const freqFactor = Math.sin(((i + 1) / (barCount + 1)) * Math.PI);
                const wave1 = Math.sin(now * 5.2 + i * 0.35) * 0.38 + 0.38;
                const wave2 = Math.cos(now * 8.4 - i * 0.5) * 0.28 + 0.28;
                const wave3 = Math.sin(now * 3.1 + i * 0.18) * 0.22 + 0.22;
                targetHeightPct = Math.min(0.96, Math.max(0.08, (wave1 + wave2 + wave3) * 0.45 * freqFactor + 0.12));
            } else {
                targetHeightPct = 0.05;
            }

            window._fsBarHeights[i] += (targetHeightPct - window._fsBarHeights[i]) * 0.2;
            const currentHeight = Math.max(4, window._fsBarHeights[i] * (height - 24));

            if (currentHeight > window._fsPeakCaps[i]) {
                window._fsPeakCaps[i] = currentHeight;
            } else {
                window._fsPeakCaps[i] = Math.max(4, window._fsPeakCaps[i] - 1.4);
            }

            const x = padding + i * (barWidth + padding);
            const y = height - currentHeight;

            const gradient = ctx.createLinearGradient(0, height, 0, y);
            gradient.addColorStop(0, "rgba(34, 211, 238, 0.25)");
            gradient.addColorStop(0.5, "#22d3ee");
            gradient.addColorStop(1, "#c084fc");

            ctx.fillStyle = gradient;
            ctx.beginPath();
            if (typeof ctx.roundRect === "function") {
                ctx.roundRect(x, y, barWidth, currentHeight, [3, 3, 0, 0]);
            } else {
                ctx.rect(x, y, barWidth, currentHeight);
            }
            ctx.fill();

            // Peak cap line
            const capY = height - window._fsPeakCaps[i] - 3;
            ctx.fillStyle = "#ffffff";
            ctx.shadowColor = "#22d3ee";
            ctx.shadowBlur = 4;
            ctx.fillRect(x, Math.max(2, capY), barWidth, 2);
            ctx.shadowBlur = 0;
        }

        ctx.restore();
        fsVisualizerAnimFrame = requestAnimationFrame(drawFrame);
    }

    drawFrame();
}

function openFullscreenVisualizer() {
    const fsOverlay = document.getElementById("fullscreen-overlay");
    if (!fsOverlay) return;

    fsOverlay.classList.add("fs-active");
    fsOverlay.style.display = "flex";

    const track = state.activeTrackId ? trackDetailsCache[state.activeTrackId] : null;
    if (track && typeof syncFullscreenVisualizer === "function") {
        syncFullscreenVisualizer(track);
    }
    startFullscreenVisualizerSpectrum();

    if (typeof saveServerState === "function") {
        saveServerState("player-fullscreen-active", "true");
    }
}

function closeRightPanel() {
    const container = document.getElementById("app-container");
    const detailsDrawer = document.getElementById("details-drawer");
    if (container) container.classList.remove("drawer-open");
    if (detailsDrawer) detailsDrawer.classList.remove("drawer-open");
    const btnTogglePlayer = document.getElementById("btn-toggle-player");
    if (btnTogglePlayer) btnTogglePlayer.classList.remove("active");
    if (typeof saveServerState === "function") saveServerState("player-drawer-open", "false");
}

document.addEventListener("click", (e) => {
    const visBtn = e.target.closest("#btn-fullscreen-toggle, #btn-drawer-fullscreen, #btn-drawer-fullscreen-top, .btn-fullscreen");
    if (visBtn) {
        e.preventDefault();
        openFullscreenVisualizer();
        return;
    }

    const closeFsBtn = e.target.closest("#btn-fullscreen-close, .fullscreen-close-btn");
    if (closeFsBtn) {
        e.preventDefault();
        const fsOverlay = document.getElementById("fullscreen-overlay");
        if (fsOverlay) {
            fsOverlay.classList.remove("fs-active");
            fsOverlay.style.display = "none";
        }
        if (typeof saveServerState === "function") saveServerState("player-fullscreen-active", "false");
        return;
    }

    const closeRightBtn = e.target.closest("#btn-close-drawer-right, #btn-close-drawer");
    if (closeRightBtn) {
        e.preventDefault();
        closeRightPanel();
        return;
    }
});

document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
        const fsOverlay = document.getElementById("fullscreen-overlay");
        if (fsOverlay && (fsOverlay.style.display === "flex" || fsOverlay.classList.contains("fs-active"))) {
            fsOverlay.classList.remove("fs-active");
            fsOverlay.style.display = "none";
            if (typeof saveServerState === "function") saveServerState("player-fullscreen-active", "false");
        }
    }
});

// Helper to get next track in queue
function getNextTrack() {
    if (state.activePlaylist.length === 0) return null;

    if (state.repeatMode === "album" && state.currentAlbumTracks && state.currentAlbumTracks.length > 0) {
        if (state.shuffleMode) {
            let pool = state.currentAlbumTracks.filter(t => Number(t.id) !== Number(state.activeTrackId));
            if (pool.length === 0) pool = state.currentAlbumTracks;
            return pool[Math.floor(Math.random() * pool.length)];
        } else {
            const albIdx = state.currentAlbumTracks.findIndex(t => Number(t.id) === Number(state.activeTrackId));
            const nextAlbIdx = albIdx !== -1 ? (albIdx + 1) % state.currentAlbumTracks.length : 0;
            return state.currentAlbumTracks[nextAlbIdx];
        }
    }

    let nextIdx = -1;
    const currentTrackIndex = state.activePlaylist.findIndex(t => Number(t.id) === Number(state.activeTrackId));

    if (state.shuffleMode) {
        const currentShufflePos = state.shuffleIndices.indexOf(currentTrackIndex);
        if (currentShufflePos !== -1 && currentShufflePos + 1 < state.shuffleIndices.length) {
            nextIdx = state.shuffleIndices[currentShufflePos + 1];
        } else if (state.repeatMode === "all") {
            nextIdx = state.shuffleIndices[0];
        }
    } else {
        if (currentTrackIndex !== -1 && currentTrackIndex + 1 < state.activePlaylist.length) {
            nextIdx = currentTrackIndex + 1;
        } else if (state.repeatMode === "all") {
            nextIdx = 0;
        }
    }
    return nextIdx !== -1 ? state.activePlaylist[nextIdx] : null;
}

function syncFullscreenVisualizer(track) {
    if (!track) return;

    const fsTitle = document.getElementById("fs-track-title") || document.getElementById("fs-title");
    const fsArtist = document.getElementById("fs-track-artist") || document.getElementById("fs-artist");
    if (fsTitle) fsTitle.textContent = track.title || "Unknown Track";
    if (fsArtist) fsArtist.textContent = track.artist || "Unknown Artist";

    // Sync next song
    const nextTrack = getNextTrack();
    const nextUpText = document.getElementById("fs-next-up-text");
    if (nextUpText) {
        if (nextTrack) {
            nextUpText.textContent = `"${nextTrack.title}" by ${nextTrack.artist}`;
        } else {
            nextUpText.textContent = "End of Playlist";
        }
    }

    // Copy cover artwork
    const fsArt = document.getElementById("fs-album-art");
    const fsInitials = document.getElementById("fs-album-initials");

    if (fsInitials) {
        fsInitials.textContent = (track.title || "UT").substring(0, 2);
        fsInitials.style.display = "block";
    }
    if (fsArt) {
        fsArt.style.display = "none";
        fsArt.src = "";
    }

    // Fetch directly from the cover art API to ensure visualizer image loads
    const artUrl = `/api/art?id=${track.album_art_id || track.id}`;
    const tempImg = new Image();
    tempImg.onload = () => {
        if (fsArt) {
            fsArt.src = artUrl;
            fsArt.style.display = "block";
        }
        if (fsInitials) fsInitials.style.display = "none";
    };
    tempImg.onerror = () => {
        if (fsArt) fsArt.style.display = "none";
        if (fsInitials) fsInitials.style.display = "block";
    };
    tempImg.src = artUrl;

    // Copy Spec labels
    const fsStrings = document.getElementById("fs-val-strings");
    const fsDrums = document.getElementById("fs-val-drums");
    const fsSmoothness = document.getElementById("fs-val-smoothness");
    const fsDr = document.getElementById("fs-val-dr");

    if (fsStrings) fsStrings.textContent = `${Math.round((track.strings_score || 0) * 100)}%`;
    if (fsDrums) fsDrums.textContent = `${Math.round((track.drums_score || 0) * 100)}%`;
    if (fsSmoothness) fsSmoothness.textContent = (track.audio_smoothness || 0.5).toFixed(3);
    if (fsDr) fsDr.textContent = track.dynamic_range ? `${track.dynamic_range.toFixed(2)} dB` : "N/A";

    const fsLrcStatus = document.getElementById("fs-val-lrc-status");
    if (fsLrcStatus) {
        fsLrcStatus.textContent = track.lrc_content ? "Synchronized" : "Instrumental / No LRC";
    }

    // Technical Audio Telemetry Card populate
    const statFormat = document.getElementById("fs-stat-format");
    const statSampleRate = document.getElementById("fs-stat-samplerate");
    const statBitrate = document.getElementById("fs-stat-bitrate");
    const statPeak = document.getElementById("fs-stat-peak");
    const statPlaySkips = document.getElementById("fs-stat-playskips");
    const statAffinity = document.getElementById("fs-stat-affinity");

    const fmt = (track.format || track.extension || "FLAC").toUpperCase();
    const sr = track.sample_rate ? `${(track.sample_rate / 1000).toFixed(1)} kHz` : "44.1 kHz";
    const bd = track.bit_depth ? `${track.bit_depth}-bit` : "16-bit";
    const br = track.bitrate ? `${Math.round(track.bitrate / 1000)} kbps` : (fmt === "FLAC" || fmt === "WAV" ? "1411 kbps (Lossless)" : "320 kbps");
    const plays = track.play_count || 0;
    const skips = track.skip_count || 0;
    const fav = track.favorite_count || 0;
    const dis = track.disliked || 0;
    const affinityVal = (fav * 5.0 + plays * 1.0 - skips * 2.0 - dis * 10.0).toFixed(1);

    if (statFormat) statFormat.textContent = fmt;
    if (statSampleRate) statSampleRate.textContent = `${sr} / ${bd}`;
    if (statBitrate) statBitrate.textContent = br;
    if (statPeak) statPeak.textContent = "L: -0.8 dB | R: -1.1 dB";
    if (statPlaySkips) statPlaySkips.textContent = `${plays} plays / ${skips} skips`;
    if (statAffinity) statAffinity.textContent = `${affinityVal >= 0 ? '+' : ''}${affinityVal} (${dis ? 'Disliked' : (fav ? 'Favorited' : 'Standard')})`;

    // Inject charts
    renderRadarChart(track, "fs-radar-container");
    renderEmotionArcChart(track, "fs-emotion-container");
    startFullscreenVisualizerSpectrum();

    // Clone Section timeline
    const fsTimeline = document.getElementById("fs-timeline-container");
    const timelineOrig = document.getElementById("timeline-container-v2");
    if (fsTimeline && timelineOrig) {
        fsTimeline.innerHTML = timelineOrig.innerHTML.replace(/timeline-sec-/g, "fs-timeline-sec-");
    }

    // Clone lyrics block over with fullscreen ids
    const fsLyrics = document.getElementById("fs-lyrics-content");

    if (state.lyricLines.length > 0) {
        let fsHtml = "";
        state.lyricLines.forEach(line => {
            fsHtml += `<div class="lyrics-line" id="fs-lyric-line-${line.index}" data-time="${line.time}">${escapeHtml(line.text || "🎵")}</div>`;
        });
        fsLyrics.innerHTML = fsHtml;
    } else {
        fsLyrics.innerHTML = `<div class="no-lyrics">No lyrics loaded.</div>`;
    }

    lastActiveLyricIdx = -1;
}

const fsToggleBtn = document.getElementById("btn-fullscreen-toggle");
const fsCloseBtn = document.getElementById("btn-fullscreen-close");

if (fsToggleBtn) {
    fsToggleBtn.addEventListener("click", () => {
        openFullscreenVisualizer();
    });
}

if (fsCloseBtn) {
    fsCloseBtn.addEventListener("click", () => {
        const fsOverlay = document.getElementById("fullscreen-overlay");
        if (fsOverlay) fsOverlay.classList.remove("fs-active");
        if (typeof saveServerState === "function") saveServerState("player-fullscreen-active", "false");
    });
}

// Fullscreen player controls click handlers
const fsBtnPlay = document.getElementById("fs-btn-play-pause");
if (fsBtnPlay) {
    fsBtnPlay.addEventListener("click", () => {
        if (playPauseBtn) playPauseBtn.click();
    });
}

const fsBtnPrev = document.getElementById("fs-btn-prev");
if (fsBtnPrev) {
    fsBtnPrev.addEventListener("click", () => {
        if (rewindBtn) rewindBtn.click();
    });
}

const fsBtnNext = document.getElementById("fs-btn-next");
if (fsBtnNext) {
    fsBtnNext.addEventListener("click", () => {
        if (forwardBtn) forwardBtn.click();
    });
}

// Fullscreen player shuffle & repeat controls are bound uniformly in bindPlaybackControls()

// Global Playlist queue play control loops
function generateShuffleIndices() {
    state.shuffleIndices = [];
    if (state.activePlaylist.length === 0) return;

    const validIndices = [];
    for (let i = 0; i < state.activePlaylist.length; i++) {
        const tr = state.activePlaylist[i];
        if (!tr || (!tr.disliked && Number(tr.disliked) !== 1)) {
            validIndices.push(i);
        }
    }
    if (validIndices.length === 0) {
        for (let i = 0; i < state.activePlaylist.length; i++) validIndices.push(i);
    }

    state.shuffleIndices = [...validIndices];

    if (state.shuffleMode) {
        for (let i = state.shuffleIndices.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            const temp = state.shuffleIndices[i];
            state.shuffleIndices[i] = state.shuffleIndices[j];
            state.shuffleIndices[j] = temp;
        }
    }
}

function handleTrackEnded() {
    if (state.repeatMode === "one") {
        playAudio();
    } else {
        playNextTrack();
    }
}

async function playNextTrack() {
    if (state.activePlaylist.length === 0) return;

    let currentIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(state.activeTrackId));
    if (currentIdx === -1) currentIdx = 0;

    let nextIdx = 0;
    if (state.repeatMode === "album") {
        let pool = state.currentAlbumTracks || [];
        if (pool.length === 0) {
            let currentTrack = state.activePlaylist.find(t => Number(t.id) === Number(state.activeTrackId));
            let albumName = currentTrack ? currentTrack.album : "";
            if (!albumName) {
                try {
                    const res = await fetch(`/api/track?id=${state.activeTrackId}`);
                    if (res.ok) {
                        const track = await res.json();
                        albumName = track.album;
                    }
                } catch (e) { console.error(e); }
            }
            if (albumName) {
                try {
                    const res = await fetch(`/api/remote/tracks?album=${encodeURIComponent(albumName)}`);
                    if (res.ok) {
                        pool = await res.json();
                        state.currentAlbumTracks = pool;
                    }
                } catch (e) { console.error(e); }
            }
        }
        if (pool.length > 0) {
            let filterPool = pool.filter(t => (!t.disliked && Number(t.disliked) !== 1));
            if (filterPool.length === 0) filterPool = pool;
            let chosenTrack = null;
            if (state.shuffleMode) {
                let cand = filterPool.filter(t => Number(t.id) !== Number(state.activeTrackId));
                if (cand.length === 0) cand = filterPool;
                chosenTrack = cand[Math.floor(Math.random() * cand.length)];
            } else {
                const albIdx = filterPool.findIndex(t => Number(t.id) === Number(state.activeTrackId));
                const nextAlbIdx = albIdx !== -1 ? (albIdx + 1) % filterPool.length : 0;
                chosenTrack = filterPool[nextAlbIdx];
            }
            if (chosenTrack) {
                playImmediate(chosenTrack.id);
                return;
            }
        }
    }

    if (state.shuffleMode === "ai") {
        let track = trackDetailsCache[state.activeTrackId];
        if (!track) {
            try {
                const res = await fetch(`/api/track?id=${state.activeTrackId}`);
                if (res.ok) track = await res.json();
            } catch (e) { console.error(e); }
        }
        const validSims = (track && track.similar_tracks) ? track.similar_tracks.filter(t => !t.disliked && Number(t.disliked) !== 1) : [];
        if (validSims.length > 0) {
            const randomSim = validSims[Math.floor(Math.random() * validSims.length)];
            const existingIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(randomSim.id));
            if (existingIdx !== -1) {
                nextIdx = existingIdx;
            } else {
                const simTrackObj = { id: randomSim.id, title: randomSim.title, artist: randomSim.artist, album: randomSim.album || "", duration: randomSim.duration || 180, disliked: 0 };
                state.activePlaylist.splice(currentIdx + 1, 0, simTrackObj);
                nextIdx = currentIdx + 1;
            }
            generateShuffleIndices();
        } else {
            const orderIdx = state.shuffleIndices.indexOf(currentIdx);
            if (orderIdx !== -1 && orderIdx < state.shuffleIndices.length - 1) {
                nextIdx = state.shuffleIndices[orderIdx + 1];
            } else {
                if (state.repeatMode === "none") return;
                nextIdx = state.shuffleIndices[0];
            }
        }
    } else if (state.shuffleMode === "melody") {
        try {
            const res = await fetch(`/api/track/melody_matches?id=${state.activeTrackId}`);
            if (res.ok) {
                const data = await res.json();
                const validMels = (data && data.matches) ? data.matches.filter(t => !t.disliked && Number(t.disliked) !== 1) : [];
                if (validMels.length > 0) {
                    const randomMel = validMels[Math.floor(Math.random() * validMels.length)];
                    const existingIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(randomMel.id));
                    if (existingIdx !== -1) {
                        nextIdx = existingIdx;
                    } else {
                        const melTrackObj = { id: randomMel.id, title: randomMel.title, artist: randomMel.artist, album: randomMel.album || "", duration: randomMel.duration || 180, disliked: 0 };
                        state.activePlaylist.splice(currentIdx + 1, 0, melTrackObj);
                        nextIdx = currentIdx + 1;
                    }
                    generateShuffleIndices();
                } else {
                    const orderIdx = state.shuffleIndices.indexOf(currentIdx);
                    if (orderIdx !== -1 && orderIdx < state.shuffleIndices.length - 1) {
                        nextIdx = state.shuffleIndices[orderIdx + 1];
                    } else {
                        if (state.repeatMode === "none") return;
                        nextIdx = state.shuffleIndices[0];
                    }
                }
            }
        } catch (e) {
            console.error(e);
            const orderIdx = state.shuffleIndices.indexOf(currentIdx);
            if (orderIdx !== -1 && orderIdx < state.shuffleIndices.length - 1) {
                nextIdx = state.shuffleIndices[orderIdx + 1];
            } else {
                if (state.repeatMode === "none") return;
                nextIdx = state.shuffleIndices[0];
            }
        }

    } else if (state.shuffleMode) {
        const orderIdx = state.shuffleIndices.indexOf(currentIdx);
        if (orderIdx !== -1 && orderIdx < state.activePlaylist.length - 1) {
            nextIdx = state.shuffleIndices[orderIdx + 1];
        } else {
            if (state.repeatMode === "none") return;
            nextIdx = state.shuffleIndices[0];
        }
    } else {
        if (currentIdx < state.activePlaylist.length - 1) {
            nextIdx = currentIdx + 1;
        } else {
            if (state.repeatMode === "none") return;
            nextIdx = 0;
        }
    }

    const nextTrack = state.activePlaylist[nextIdx];

    const itemIndex = nextIdx;
    const targetPage = Math.floor(itemIndex / state.limit) + 1;
    if (state.activeWorkspace === "workspace-library" && state.currentPage !== targetPage) {
        state.currentPage = targetPage;
        await loadTracks();
    }

    selectTrack(nextTrack.id, true);
}

async function playPreviousTrack() {
    if (state.activePlaylist.length === 0) return;

    let currentIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(state.activeTrackId));
    if (currentIdx === -1) currentIdx = 0;

    let prevIdx = 0;
    if (state.shuffleMode) {
        const orderIdx = state.shuffleIndices.indexOf(currentIdx);
        if (orderIdx !== -1 && orderIdx > 0) {
            prevIdx = state.shuffleIndices[orderIdx - 1];
        } else {
            prevIdx = state.shuffleIndices[state.activePlaylist.length - 1];
        }
    } else {
        if (currentIdx > 0) {
            prevIdx = currentIdx - 1;
        } else {
            prevIdx = state.activePlaylist.length - 1;
        }
    }

    const prevTrack = state.activePlaylist[prevIdx];

    // Auto page swapping
    const itemIndex = prevIdx;
    const targetPage = Math.floor(itemIndex / state.limit) + 1;
    if (state.activeWorkspace === "workspace-library" && state.currentPage !== targetPage) {
        state.currentPage = targetPage;
        await loadTracks();
    }

    selectTrack(prevTrack.id, true);
}

// --- SHUFFLE CONTROLS ---
async function toggleShuffleState() {
    const isShuffleActive = state.shuffleMode !== false && state.shuffleMode !== "none";
    if (isShuffleActive) {
        state.shuffleMode = false;
    } else {
        state.shuffleMode = state.lastShuffleMode || "normal";
    }

    saveServerState("player-shuffle-mode", state.shuffleMode);
    if (state.lastShuffleMode) saveServerState("player-last-shuffle-mode", state.lastShuffleMode);
    syncShuffleModeUI();
    generateShuffleIndices();

    try {
        await fetch("/api/player/mode", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ shuffle_mode: state.shuffleMode })
        });
    } catch (e) {
        console.error("Error updating server shuffle mode:", e);
    }
}

async function cycleShuffleMode() {
    const currentMode = state.lastShuffleMode || (typeof state.shuffleMode === "string" ? state.shuffleMode : "normal");
    let nextMode = "normal";
    if (currentMode === "normal" || currentMode === true) nextMode = "ai";
    else if (currentMode === "ai") nextMode = "melody";
    else if (currentMode === "melody") nextMode = "normal";

    state.shuffleMode = nextMode;
    state.lastShuffleMode = nextMode;

    saveServerState("player-shuffle-mode", state.shuffleMode);
    saveServerState("player-last-shuffle-mode", state.lastShuffleMode);
    syncShuffleModeUI();
    generateShuffleIndices();

    try {
        await fetch("/api/player/mode", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ shuffle_mode: state.shuffleMode })
        });
    } catch (e) {
        console.error("Error updating server shuffle mode:", e);
    }
}

// --- REPEAT CONTROLS ---
async function toggleRepeatState() {
    const isRepeatActive = state.repeatMode !== false && state.repeatMode !== "none";
    if (isRepeatActive) {
        state.repeatMode = "none";
    } else {
        state.repeatMode = state.lastRepeatMode || "all";
    }

    saveServerState("player-repeat-mode", state.repeatMode);
    syncRepeatModeUI();

    try {
        await fetch("/api/player/mode", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ repeat_mode: state.repeatMode })
        });
    } catch (e) {
        console.error("Error updating server repeat mode:", e);
    }
}

async function cycleRepeatMode() {
    const currentMode = (state.repeatMode !== false && state.repeatMode !== "none")
        ? state.repeatMode
        : (state.lastRepeatMode || "all");

    let nextMode = "all";
    if (currentMode === "all") nextMode = "album";
    else if (currentMode === "album") nextMode = "artist";
    else if (currentMode === "artist") nextMode = "one";
    else if (currentMode === "one") nextMode = "all";

    state.repeatMode = nextMode;
    state.lastRepeatMode = nextMode;

    saveServerState("player-repeat-mode", state.repeatMode);
    syncRepeatModeUI();

    try {
        await fetch("/api/player/mode", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ repeat_mode: state.repeatMode })
        });
    } catch (e) {
        console.error("Error updating server repeat mode:", e);
    }
}

function bindPlaybackControls() {
    ["audio-btn-shuffle", "mini-btn-shuffle", "fs-btn-shuffle"].forEach(id => {
        const el = document.getElementById(id);
        if (el && !el.dataset.bound) {
            el.dataset.bound = "true";
            el.addEventListener("click", (e) => { e.stopPropagation(); toggleShuffleState(); });
        }
    });

    ["audio-btn-shuffle-mode", "mini-btn-shuffle-mode", "fs-btn-shuffle-mode", "overlay-btn-shuffle-mode"].forEach(id => {
        const el = document.getElementById(id);
        if (el && !el.dataset.bound) {
            el.dataset.bound = "true";
            el.addEventListener("click", (e) => { e.stopPropagation(); cycleShuffleMode(); });
        }
    });

    ["audio-btn-repeat", "mini-btn-repeat", "fs-btn-repeat"].forEach(id => {
        const el = document.getElementById(id);
        if (el && !el.dataset.bound) {
            el.dataset.bound = "true";
            el.addEventListener("click", (e) => { e.stopPropagation(); toggleRepeatState(); });
        }
    });

    ["audio-btn-repeat-mode", "mini-btn-repeat-mode", "fs-btn-repeat-mode", "overlay-btn-repeat-mode"].forEach(id => {
        const el = document.getElementById(id);
        if (el && !el.dataset.bound) {
            el.dataset.bound = "true";
            el.addEventListener("click", (e) => { e.stopPropagation(); cycleRepeatMode(); });
        }
    });

    ["btn-drawer-dislike", "mini-btn-dislike", "fs-btn-dislike", "overlay-btn-dislike"].forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            el.onclick = (e) => {
                e.stopPropagation();
                const activeId = state.activeTrackId || (typeof remote_status !== "undefined" && remote_status.track_id);
                if (activeId) toggleDislike(activeId);
            };
        }
    });

    ["btn-drawer-favorite", "mini-btn-favorite", "fs-btn-favorite", "overlay-btn-favorite"].forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            el.onclick = (e) => {
                e.stopPropagation();
                const activeId = state.activeTrackId || (typeof remote_status !== "undefined" && remote_status.track_id);
                if (activeId) toggleFavorite(activeId);
            };
        }
    });

    ["audio-btn-dsp-toggle", "mini-btn-dsp-toggle", "overlay-btn-dsp-toggle"].forEach(id => {
        const el = document.getElementById(id);
        if (el && !el.dataset.bound) {
            el.dataset.bound = "true";
            el.addEventListener("click", (e) => { e.stopPropagation(); toggleMasterDSP(); });
        }
    });

    const targetBtn = document.getElementById("btn-toggle-vol-target");
    if (targetBtn && !targetBtn.dataset.bound) {
        targetBtn.dataset.bound = "true";
        targetBtn.addEventListener("click", (e) => { e.stopPropagation(); toggleVolumeKeysTarget(); });
    }
    syncVolumeKeysTargetUI();
}

if (document.readyState === "complete" || document.readyState === "interactive") {
    bindPlaybackControls();
} else {
    document.addEventListener("DOMContentLoaded", bindPlaybackControls);
}

function syncVolumeKeysTargetUI() {
    const curTarget = state.volumeKeysTarget || "exclusive";
    const btn = document.getElementById("btn-toggle-vol-target");
    const icon = document.getElementById("icon-vol-target");
    const label = document.getElementById("label-vol-target");
    const select = document.getElementById("settings-select-volume-target");

    if (select) {
        select.value = curTarget;
    }

    if (btn) {
        if (curTarget === "null") {
            btn.style.background = "rgba(56, 189, 248, 0.15)";
            btn.style.borderColor = "rgba(56, 189, 248, 0.4)";
            btn.style.color = "#38bdf8";
            if (icon) icon.className = "fa-solid fa-desktop";
            if (label) label.textContent = "HW: NULL/SYS";
            btn.title = "Hardware Volume Keys (Vol Up/Down/Mute) control Null Target Device (System / YouTube). Click to toggle to WASAPI Exclusive Music Player.";
        } else {
            btn.style.background = "rgba(16, 185, 129, 0.15)";
            btn.style.borderColor = "rgba(16, 185, 129, 0.4)";
            btn.style.color = "#10b981";
            if (icon) icon.className = "fa-solid fa-headphones";
            if (label) label.textContent = "HW: WASAPI";
            btn.title = "Hardware Volume Keys (Vol Up/Down/Mute) control Sonar WASAPI Exclusive Music Player. Click to toggle to Null Target Device.";
        }
    }
}

async function setVolumeKeysTarget(targetVal) {
    state.volumeKeysTarget = targetVal;
    syncVolumeKeysTargetUI();
    saveServerState("dsp-volume_keys_target", targetVal);
    await flushServerState();
    if (typeof showNotification === "function") {
        showNotification(targetVal === "null" ? "🔇 Hardware Volume Keys now control Null Target Device (System / YouTube)" : "🎵 Hardware Volume Keys now control Sonar WASAPI Exclusive Music Player");
    }
}

async function toggleVolumeKeysTarget() {
    const nextTarget = (state.volumeKeysTarget === "null") ? "exclusive" : "null";
    await setVolumeKeysTarget(nextTarget);
}

function syncRepeatModeUI() {
    const isRepeatActive = state.repeatMode !== false && state.repeatMode !== "none";
    const activeMode = isRepeatActive ? state.repeatMode : (state.lastRepeatMode || "all");

    // 1. Toggle Buttons (ON / OFF)
    const rToggleBtns = [
        document.getElementById("audio-btn-repeat"),
        document.getElementById("fs-btn-repeat"),
        document.getElementById("mini-btn-repeat")
    ].filter(Boolean);

    rToggleBtns.forEach(btn => {
        btn.className = isRepeatActive ? "control-btn playlist-opt-btn active-repeat" : "control-btn playlist-opt-btn";
        btn.style.opacity = isRepeatActive ? "1" : "0.35";
        btn.innerHTML = `<i class="fa-solid fa-repeat"></i>`;
        btn.title = isRepeatActive ? `Repeat: ON (${activeMode.toUpperCase()}) - Click to turn OFF` : `Repeat: OFF - Click to turn ON`;
    });

    // 2. Mode Selector Buttons (Cycles mode: All -> Album -> Artist -> One)
    const rModeBtns = [
        document.getElementById("audio-btn-repeat-mode"),
        document.getElementById("mini-btn-repeat-mode"),
        document.getElementById("fs-btn-repeat-mode")
    ].filter(Boolean);

    let modeIcon = "fa-solid fa-repeat";
    let modeLabel = "All Tracks";
    if (activeMode === "album") {
        modeIcon = "fa-solid fa-compact-disc";
        modeLabel = "Album";
    } else if (activeMode === "artist") {
        modeIcon = "fa-solid fa-user";
        modeLabel = "Artist";
    } else if (activeMode === "one") {
        modeIcon = "fa-solid fa-rotate-left";
        modeLabel = "One Track";
    }

    rModeBtns.forEach(btn => {
        btn.style.display = "inline-flex";
        btn.className = isRepeatActive ? "control-btn-sm mode-selector-btn active-mode" : "control-btn-sm mode-selector-btn";
        btn.innerHTML = `<i class="${modeIcon}"></i>`;
        btn.title = `Repeat Mode: ${modeLabel} (Click to change mode)`;
    });
}

function syncShuffleModeUI() {
    const isShuffleActive = state.shuffleMode !== false && state.shuffleMode !== "none";
    const activeMode = state.lastShuffleMode || (typeof state.shuffleMode === "string" ? state.shuffleMode : "normal");

    // 1. Toggle Buttons (ON / OFF)
    const sToggleBtns = [
        document.getElementById("audio-btn-shuffle"),
        document.getElementById("fs-btn-shuffle"),
        document.getElementById("mini-btn-shuffle")
    ].filter(Boolean);

    sToggleBtns.forEach(btn => {
        btn.className = isShuffleActive ? "control-btn playlist-opt-btn active-shuffle" : "control-btn playlist-opt-btn";
        btn.style.opacity = isShuffleActive ? "1" : "0.35";
        btn.innerHTML = `<i class="fa-solid fa-shuffle"></i>`;
        btn.title = isShuffleActive ? `Shuffle: ON (${activeMode.toUpperCase()}) - Click to turn OFF` : `Shuffle: OFF - Click to turn ON`;
    });

    // 2. Mode Selector Buttons (Cycles mode: Normal -> AI -> Melody)
    const sModeBtns = [
        document.getElementById("audio-btn-shuffle-mode"),
        document.getElementById("mini-btn-shuffle-mode"),
        document.getElementById("fs-btn-shuffle-mode")
    ].filter(Boolean);

    let modeIcon = "fa-solid fa-shuffle";
    let modeLabel = "Standard";
    if (activeMode === "ai") {
        modeIcon = "fa-solid fa-brain";
        modeLabel = "AI Similarity";
    } else if (activeMode === "melody") {
        modeIcon = "fa-solid fa-music";
        modeLabel = "Melody DTW";
    }

    sModeBtns.forEach(btn => {
        btn.style.display = "inline-flex";
        btn.className = isShuffleActive ? "control-btn-sm mode-selector-btn active-mode" : "control-btn-sm mode-selector-btn";
        btn.innerHTML = `<i class="${modeIcon}"></i>`;
        btn.title = `Shuffle Mode: ${modeLabel} (Click to change mode)`;
    });
}

// Playlist Builder workspace loader and selector wires
function loadPlaylistBuilderTracks() {
    const pbTbody = document.getElementById("pb-tracks-tbody");
    if (!pbTbody) return;

    pbTbody.innerHTML = `<tr><td colspan="7" class="table-loading"><i class="fa-solid fa-spinner fa-spin"></i> Fetching playlist tracks...</td></tr>`;

    const params = new URLSearchParams({
        search: state.pbSearchQuery || "",
        vocal: state.pbVocalFilter || "",
        character: state.pbCharacterFilter || "",
        key: state.pbKeyFilter || "",
        scale: state.pbScaleFilter || "",
        emotion: state.pbEmotionFilter || "",
        strings: state.pbStringsFilter || "",
        keyboards: state.pbKeyboardsFilter || "",
        piano: state.pbPianoFilter || "",
        drums: state.pbDrumsFilter || "",
        complexity: state.pbComplexityFilter || "",
        choir: state.pbChoirFilter || "",
        guitar: state.pbGuitarFilter || "",
        bass: state.pbBassFilter || "",
        winds: state.pbWindsFilter || "",
        synth: state.pbSynthFilter || "",
        brass: state.pbBrassFilter || "",
        dreaminess: state.pbDreaminessFilter || "",
        epicness: state.pbEpicnessFilter || "",
        cinematicness: state.pbCinematicnessFilter || "",
        electronicness: state.pbElectronicnessFilter || "",
        nostalgia: state.pbNostalgiaFilter || "",
        bpm: state.pbBpmFilter || "",
        limit: 100000 // load all for checkboxes
    });

    fetch(`/api/tracks?${params.toString()}`)
        .then(res => res.json())
        .then(data => {
            const tracks = data.tracks || [];
            if (tracks.length === 0) {
                pbTbody.innerHTML = `<tr><td colspan="7" class="table-empty">No matching tracks in database.</td></tr>`;
                return;
            }

            let html = "";
            tracks.forEach(t => {
                const isChecked = state.selectedTrackIds.has(t.id) ? "checked" : "";
                html += `
                    <tr>
                        <td style="text-align:center;">
                            <label class="checkbox-container" style="justify-content: center; margin: 0; display: inline-flex;">
                                <input type="checkbox" class="pb-cb" data-id="${t.id}" ${isChecked}>
                                <span class="checkmark"></span>
                            </label>
                        </td>
                        <td style="font-weight:700; color:var(--text-high);">${escapeHtml(t.title)}</td>
                        <td>${renderArtistLinks(t.artist)}</td>
                        <td>${escapeHtml(t.album)}</td>
                        <td class="col-center">${formatDuration(t.duration)}</td>
                        <td class="col-center">${t.vocal_status === 'vocal' ? 'Vocal' : 'BGM'}</td>
                        <td class="col-center">${t.audio_smoothness !== null && t.audio_smoothness !== undefined ? t.audio_smoothness.toFixed(2) : "0.00"}</td>
                    </tr>
                `;
            });
            pbTbody.innerHTML = html;

            // Wire checkbox change listeners
            pbTbody.querySelectorAll(".pb-cb").forEach(cb => {
                cb.addEventListener("change", () => {
                    const id = parseInt(cb.getAttribute("data-id"));
                    if (cb.checked) {
                        state.selectedTrackIds.add(id);
                    } else {
                        state.selectedTrackIds.delete(id);
                    }
                    updateSelectionUI();
                });
            });
            updateSelectionUI();
        })
        .catch(err => {
            console.error("Playlist Builder tracks error:", err);
            pbTbody.innerHTML = `<tr><td colspan="7" class="table-empty">Error loading builder database.</td></tr>`;
        });
}

// Wire Playlist Builder bulk controls
document.getElementById("pb-btn-select-all").addEventListener("click", () => {
    document.querySelectorAll(".pb-cb").forEach(cb => {
        if (!cb.checked) {
            cb.checked = true;
            cb.dispatchEvent(new Event("change"));
        }
    });
});
document.getElementById("pb-btn-select-vocals").addEventListener("click", () => {
    document.querySelectorAll(".pb-cb").forEach(cb => {
        const row = cb.closest("tr");
        const isVocal = row.querySelectorAll("td")[5].textContent === "Vocal";
        if (isVocal && !cb.checked) {
            cb.checked = true;
            cb.dispatchEvent(new Event("change"));
        }
    });
});
document.getElementById("pb-btn-select-bgm").addEventListener("click", () => {
    document.querySelectorAll(".pb-cb").forEach(cb => {
        const row = cb.closest("tr");
        const isBgm = row.querySelectorAll("td")[5].textContent === "BGM";
        if (isBgm && !cb.checked) {
            cb.checked = true;
            cb.dispatchEvent(new Event("change"));
        }
    });
});
document.getElementById("pb-btn-clear").addEventListener("click", () => {
    state.selectedTrackIds.clear();
    document.querySelectorAll(".pb-cb").forEach(cb => {
        if (cb.checked) {
            cb.checked = false;
            cb.dispatchEvent(new Event("change"));
        }
    });
    updateSelectionUI();
});
document.getElementById("pb-btn-export").addEventListener("click", () => {
    if (state.selectedTrackIds.size === 0) {
        alert("Please check at least one track to export.");
        return;
    }
    const formatCurrentDateTime = () => {
        const now = new Date();
        const year = now.getFullYear();
        const month = String(now.getMonth() + 1).padStart(2, '0');
        const day = String(now.getDate()).padStart(2, '0');
        const hours = String(now.getHours()).padStart(2, '0');
        const minutes = String(now.getMinutes()).padStart(2, '0');
        const seconds = String(now.getSeconds()).padStart(2, '0');
        return `${year}-${month}-${day}_${hours}-${minutes}-${seconds}`;
    };
    const name = document.getElementById("pb-playlist-name").value.trim() || formatCurrentDateTime();
    const ids = Array.from(state.selectedTrackIds).join(",");
    window.location.href = `/api/export_m3u?ids=${ids}&name=${encodeURIComponent(name)}`;
});
document.getElementById("pb-search-input").addEventListener("input", () => {
    state.pbSearchQuery = document.getElementById("pb-search-input").value;
    loadPlaylistBuilderTracks();
    saveServerState("player-filter-pbSearchQuery", state.pbSearchQuery);
});

// Render Soundtrack Theme Explorer family motif accordions
async function loadThemeExplorer() {
    console.log("loadThemeExplorer invoked");
    const container = document.getElementById("themes-explorer-container");
    if (!container) {
        console.error("themes-explorer-container element not found in DOM!");
        showDebugError("themes-explorer-container element not found in DOM!");
        return;
    }

    container.innerHTML = `<div style="padding: 20px; font-size:14px; color:var(--text-mid);"><i class="fa-solid fa-spinner fa-spin"></i> Fetching soundtrack theme families...</div>`;
    console.log("loadThemeExplorer: set spinner innerHTML");

    const themesSearchInput = document.getElementById("themes-search-input");
    const searchVal = themesSearchInput ? themesSearchInput.value.toLowerCase().trim() : "";
    console.log("loadThemeExplorer: searchVal =", searchVal);

    try {
        console.log("loadThemeExplorer: fetching /api/themes...");
        const res = await fetch("/api/themes");
        console.log("loadThemeExplorer: fetch status =", res.status);
        if (!res.ok) throw new Error(`Themes fetch failed with status: ${res.status}`);
        const themes = await res.json();
        console.log("loadThemeExplorer: parsed themes json, keys count =", Object.keys(themes).length);

        let html = "";
        const keys = Object.keys(themes);

        if (keys.length === 0) {
            container.innerHTML = `<div class="table-empty">No theme families found in database. Run a library scan to group tracks.</div>`;
            return;
        }

        keys.forEach(tfId => {
            const familyTracks = themes[tfId] || [];
            if (familyTracks.length === 0) return;

            // Check if search matches any track in this family
            if (searchVal) {
                const matchesSearch = familyTracks.some(t =>
                    (t.title || "").toLowerCase().includes(searchVal) ||
                    (t.artist || "").toLowerCase().includes(searchVal) ||
                    (t.album || "").toLowerCase().includes(searchVal)
                );
                if (!matchesSearch) return;
            }

            let nodesHtml = "";
            familyTracks.forEach((t, i) => {
                const role = i === 0 ? "Main Theme Motif" : `Variation Path #${i}`;
                const isFav = t.favorite_count > 0;
                const isFavClass = isFav ? "favorited" : "";
                const starIcon = isFav ? "fa-solid fa-star" : "fa-regular fa-star";

                const importance = typeof t.theme_importance === "number" ? t.theme_importance.toFixed(1) : "0.0";
                const similarity = typeof t.theme_similarity === "number" ? Math.round(t.theme_similarity * 100) : 0;

                nodesHtml += `
                    <div class="theme-node ${i === 0 ? 'node-main' : ''}">
                        <div class="theme-node-info theme-node-play" style="cursor:pointer;" data-id="${t.id}">
                            <span class="theme-node-title">${escapeHtml(t.title)}</span>
                            <span class="theme-node-meta">${renderArtistLinks(t.artist, false)} • ${role}</span>
                        </div>
                        <div class="theme-node-badges" style="margin-right:12px;">
                            <span class="theme-node-badge badge-importance">Imp: ${importance}</span>
                            ${i === 0 ? '' : `<span class="theme-node-badge badge-similarity">Sim: ${similarity}%</span>`}
                        </div>
                        <div class="theme-node-actions">
                            <button class="row-play-btn theme-node-play" data-id="${t.id}" style="padding:4px 8px; font-size:10px;"><i class="fa-solid fa-play"></i></button>
                            <button class="row-play-btn theme-node-queue" data-id="${t.id}" style="padding:4px 8px; font-size:10px;" title="Add to Playback Queue"><i class="fa-solid fa-plus"></i></button>
                            <button class="btn-favorite ${isFavClass}" data-id="${t.id}" style="opacity:1;" title="Toggle Favorite"><i class="${starIcon}"></i></button>
                        </div>
                    </div>
                `;
            });

            html += `
                <div class="theme-family-card collapsed" id="theme-family-card-${tfId}">
                    <div class="theme-family-header" data-card-id="theme-family-card-${tfId}" style="cursor:pointer;">
                        <div class="theme-family-title-block">
                            <i class="fa-solid fa-diagram-project theme-family-icon"></i>
                            <span class="theme-family-name">Theme Cluster Family #${tfId}</span>
                            <span class="theme-family-count">${familyTracks.length} variations</span>
                        </div>
                        <i class="fa-solid fa-chevron-down dsp-collapse-icon"></i>
                    </div>
                    <div class="theme-family-body">
                        <div class="theme-family-tree">
                            ${nodesHtml}
                        </div>
                    </div>
                </div>
            `;
        });

        container.innerHTML = html || `<div class="table-empty">No matching theme families found.</div>`;
        console.log("loadThemeExplorer: rendering completed successfully");

        // Setup event delegation once
        if (!container.dataset.listenerBound) {
            container.addEventListener("click", (e) => {
                // 1. Theme card header toggle
                const header = e.target.closest(".theme-family-header");
                if (header) {
                    const cardId = header.getAttribute("data-card-id");
                    const card = document.getElementById(cardId);
                    if (card) {
                        card.classList.toggle("collapsed");
                    }
                    return;
                }

                // 2. Add to queue button (must check before play block to avoid double actions)
                const queueEl = e.target.closest(".theme-node-queue");
                if (queueEl) {
                    e.stopPropagation();
                    const trackId = parseInt(queueEl.getAttribute("data-id"));
                    if (!isNaN(trackId)) {
                        addToQueue(trackId);
                    }
                    return;
                }

                // 3. Toggle favorite
                const favEl = e.target.closest(".btn-favorite");
                if (favEl) {
                    e.stopPropagation();
                    const trackId = parseInt(favEl.getAttribute("data-id"));
                    if (!isNaN(trackId)) {
                        toggleFavorite(trackId, favEl);
                    }
                    return;
                }

                // 4. Play track
                const playEl = e.target.closest(".theme-node-play");
                if (playEl) {
                    const trackId = parseInt(playEl.getAttribute("data-id"));
                    if (!isNaN(trackId)) {
                        playImmediate(trackId);
                    }
                    return;
                }
            });
            container.dataset.listenerBound = "true";
        }

    } catch (err) {
        console.error("Theme Explorer loading error:", err);
        container.innerHTML = `<div class="table-empty">Error loading soundtrack motif trees: ${escapeHtml(err.message)}</div>`;
        showDebugError(`Theme Explorer loading error: ${err.message}`);
    }
}


// Add track directly to playback queue array
function addToQueue(trackId) {
    if (state.activePlaylist.some(t => Number(t.id) === Number(trackId))) {
        alert("Track is already in queue.");
        return;
    }

    // Fetch detail track and push to queue list
    fetch(`/api/track?id=${trackId}`)
        .then(res => res.json())
        .then(track => {
            state.activePlaylist.push(track);
            generateShuffleIndices();
            updateQueueWidget();
            loadQueueWorkspace();
            alert(`"${track.title}" added to playback queue.`);
        })
        .catch(err => console.error("Add to queue error:", err));
}

// Render Favorites & Playback History tab tables
async function loadFavoritesWorkspace() {
    const activeTab = document.querySelector(".favorites-tabs .fav-tab-btn.active");
    if (!activeTab) return;
    const tabId = activeTab.getAttribute("data-favtab");

    const favTbody = document.getElementById("fav-tracks-tbody");
    const mpTbody = document.getElementById("mp-tracks-tbody");
    const affinityTbody = document.getElementById("affinity-tracks-tbody");

    if (tabId === "favtab-favorites" && favTbody) {
        favTbody.innerHTML = `<tr><td colspan="6" class="table-loading"><i class="fa-solid fa-spinner fa-spin"></i> Loading favorites...</td></tr>`;
        const res = await fetch("/api/tracks?favorite=true&limit=10000");
        const data = await res.json();
        const tracks = data.tracks || [];
        if (tracks.length === 0) {
            favTbody.innerHTML = `<tr><td colspan="6" class="table-empty">No favorite tracks added yet. Use the star icon ⭐ to bookmark tracks.</td></tr>`;
            return;
        }
        let html = "";
        tracks.forEach(t => {
            html += `
                <tr onclick="playImmediate('${t.id}')">
                    <td style="font-weight:700; color:var(--text-high);">${escapeHtml(t.title)}</td>
                    <td>${renderArtistLinks(t.artist)}</td>
                    <td onclick="event.stopPropagation(); filterByAlbum(decodeURIComponent('${escapeJsParam(t.album)}'))" style="cursor: pointer; color: var(--accent-purple); text-decoration: underline; text-underline-offset: 3px; font-weight: 500;">${escapeHtml(t.album)}</td>
                    <td class="col-center">${formatDuration(t.duration)}</td>
                    <td class="col-center">${t.audio_smoothness !== null && t.audio_smoothness !== undefined ? t.audio_smoothness.toFixed(2) : "0.00"}</td>
                    <td class="col-action" onclick="event.stopPropagation()">
                        <button class="row-play-btn" onclick="playImmediate('${t.id}')"><i class="fa-solid fa-play"></i></button>
                        <button class="btn-favorite favorited" onclick="toggleFavorite('${t.id}', this)"><i class="fa-solid fa-star"></i></button>
                    </td>
                </tr>
            `;
        });
        favTbody.innerHTML = html;

    } else if (tabId === "favtab-mostplayed" && mpTbody) {
        mpTbody.innerHTML = `<tr><td colspan="6" class="table-loading"><i class="fa-solid fa-spinner fa-spin"></i> Loading history...</td></tr>`;
        const res = await fetch("/api/tracks?sort=play_count&order=desc&limit=15");
        const data = await res.json();
        const tracks = data.tracks || [];
        if (tracks.length === 0) {
            mpTbody.innerHTML = `<tr><td colspan="6" class="table-empty">No tracks played yet.</td></tr>`;
            return;
        }
        let html = "";
        tracks.forEach(t => {
            html += `
                <tr onclick="playImmediate('${t.id}')">
                    <td style="font-weight:700; color:var(--text-high);">${escapeHtml(t.title)}</td>
                    <td>${renderArtistLinks(t.artist)}</td>
                    <td onclick="event.stopPropagation(); filterByAlbum(decodeURIComponent('${escapeJsParam(t.album)}'))" style="cursor: pointer; color: var(--accent-purple); text-decoration: underline; text-underline-offset: 3px; font-weight: 500;">${escapeHtml(t.album)}</td>
                    <td class="col-center" style="color:var(--accent-cyan); font-weight:800;">${t.play_count} plays</td>
                    <td class="col-center">${formatDuration(t.duration)}</td>
                    <td class="col-action" onclick="event.stopPropagation()">
                        <button class="row-play-btn" onclick="playImmediate('${t.id}')"><i class="fa-solid fa-play"></i></button>
                    </td>
                </tr>
            `;
        });
        mpTbody.innerHTML = html;

    } else if (tabId === "favtab-affinity" && affinityTbody) {
        affinityTbody.innerHTML = `<tr><td colspan="6" class="table-loading"><i class="fa-solid fa-spinner fa-spin"></i> Loading affinities...</td></tr>`;
        const res = await fetch("/api/tracks?sort=user_affinity&order=desc&limit=15");
        const data = await res.json();
        const tracks = data.tracks || [];
        let html = "";
        tracks.forEach(t => {
            html += `
                <tr onclick="playImmediate('${t.id}')">
                    <td style="font-weight:700; color:var(--text-high);">${escapeHtml(t.title)}</td>
                    <td>${renderArtistLinks(t.artist)}</td>
                    <td onclick="event.stopPropagation(); filterByAlbum(decodeURIComponent('${escapeJsParam(t.album)}'))" style="cursor: pointer; color: var(--accent-purple); text-decoration: underline; text-underline-offset: 3px; font-weight: 500;">${escapeHtml(t.album)}</td>
                    <td class="col-center" style="color:var(--accent-purple); font-weight:800;">${t.user_affinity.toFixed(1)}</td>
                    <td class="col-center">${formatDuration(t.duration)}</td>
                    <td class="col-action" onclick="event.stopPropagation()">
                        <button class="row-play-btn" onclick="playImmediate('${t.id}')"><i class="fa-solid fa-play"></i></button>
                    </td>
                </tr>
            `;
        });
        affinityTbody.innerHTML = html;

    } else if (tabId === "favtab-disliked") {
        const dislikedTbody = document.getElementById("disliked-tracks-tbody");
        if (dislikedTbody) {
            dislikedTbody.innerHTML = `<tr><td colspan="5" class="table-loading"><i class="fa-solid fa-spinner fa-spin"></i> Loading disliked tracks...</td></tr>`;
            const res = await fetch("/api/tracks?disliked=true&limit=10000");
            const data = await res.json();
            const tracks = data.tracks || [];
            if (tracks.length === 0) {
                dislikedTbody.innerHTML = `<tr><td colspan="5" class="table-empty">No disliked tracks. Disliked tracks are automatically skipped/filtered.</td></tr>`;
                return;
            }
            let html = "";
            tracks.forEach(t => {
                html += `
                    <tr style="opacity: 0.65;">
                        <td style="font-weight:700; color:var(--text-high);">${escapeHtml(t.title)}</td>
                        <td>${renderArtistLinks(t.artist)}</td>
                        <td onclick="event.stopPropagation(); filterByAlbum(decodeURIComponent('${escapeJsParam(t.album)}'))" style="cursor: pointer; color: var(--accent-purple); text-decoration: underline; text-underline-offset: 3px; font-weight: 500;">${escapeHtml(t.album)}</td>
                        <td class="col-center">${formatDuration(t.duration)}</td>
                        <td class="col-action" onclick="event.stopPropagation()">
                            <button class="btn-favorite" style="color: var(--accent-cyan);" title="Remove Dislike" onclick="toggleDislike('${t.id}')"><i class="fa-solid fa-thumbs-down"></i></button>
                        </td>
                    </tr>
                `;
            });
            dislikedTbody.innerHTML = html;
        }
    }
}

// Bind subtab buttons in Favorites & History workspace
document.querySelectorAll(".favorites-tabs .fav-tab-btn").forEach(btn => {
    btn.addEventListener("click", () => {
        document.querySelectorAll(".favorites-tabs .fav-tab-btn").forEach(b => b.classList.remove("active"));
        document.querySelectorAll(".fav-tab-content").forEach(c => c.classList.remove("active"));

        btn.classList.add("active");
        const tabId = btn.getAttribute("data-favtab");
        document.getElementById(tabId).classList.add("active");

        loadFavoritesWorkspace();
    });
});

// Themes search listener moved to setupEventListeners

// Load DSP preset list from local workstation cache
function loadDSPPresets() {
    const saved = (window.serverState?.preferences?.["dsp-presets"]);
    let presets = {};
    if (saved) {
        try { presets = JSON.parse(saved); } catch (e) { presets = {}; }
    }

    // Default preset configs
    if (!presets.default) {
        presets.default = { stereo: 100, cb_stereo: true, crossfeed: 30, cb_crossfeed: false, eq: 50, cb_eq: true, bass: 60, cb_bass: true, compressor: -24, cb_compressor: false, cb_limiter: true, warmth: 40, cb_warmth: false, reverb: 25, cb_reverb: false, vocals: 50, cb_vocals: false, air: 30, cb_air: false };
    }
    if (!presets.cinematic) {
        presets.cinematic = { stereo: 150, cb_stereo: true, crossfeed: 20, cb_crossfeed: false, eq: 70, cb_eq: true, bass: 80, cb_bass: true, compressor: -15, cb_compressor: true, cb_limiter: true, warmth: 50, cb_warmth: true, reverb: 40, cb_reverb: true, vocals: 40, cb_vocals: false, air: 50, cb_air: true };
    }
    if (!presets.latenight) {
        presets.latenight = { stereo: 110, cb_stereo: true, crossfeed: 45, cb_crossfeed: true, eq: 40, cb_eq: false, bass: 50, cb_bass: true, compressor: -30, cb_compressor: false, cb_limiter: true, warmth: 30, cb_warmth: false, reverb: 45, cb_reverb: true, vocals: 45, cb_vocals: false, air: 30, cb_air: false };
    }
    if (!presets.animevocals) {
        presets.animevocals = { stereo: 120, cb_stereo: true, crossfeed: 25, cb_crossfeed: false, eq: 60, cb_eq: true, bass: 45, cb_bass: false, compressor: -20, cb_compressor: true, cb_limiter: true, warmth: 40, cb_warmth: true, reverb: 30, cb_reverb: true, vocals: 85, cb_vocals: true, air: 60, cb_air: true };
    }

    return presets;
}

// Load Player states from localStorage
function loadPlayerState(backendStatus) {
    const savedVolume = (window.serverState?.preferences?.["player-volume"]);
    const savedLastVolume = (window.serverState?.preferences?.["player-last-volume"]);
    state.lastVolume = savedLastVolume ? parseInt(savedLastVolume) : 80;

    if (savedVolume !== null && savedVolume !== undefined && savedVolume !== "undefined") {
        const volVal = parseInt(savedVolume);
        if (!isNaN(volVal)) {
            volumeSlider.value = volVal;
            if (audio) audio.volume = volVal / 100;
            updateVolumeIcon(volVal / 100);
        }
    } else {
        // Default fallback
        if (audio) audio.volume = 0.8;
        volumeSlider.value = 80;
        updateVolumeIcon(0.8);
    }

    // Populate stored custom presets list in dropdown selector
    const presets = loadDSPPresets();
    const select = document.getElementById("dsp-preset-select");
    if (select) {
        Object.keys(presets).forEach(pId => {
            if (pId !== "default" && pId !== "cinematic" && pId !== "latenight" && pId !== "animevocals") {
                const name = pId.toUpperCase();
                const opt = document.createElement("option");
                opt.value = pId;
                opt.text = name;
                select.appendChild(opt);
            }
        });
        const defaultPresetId = window.serverState?.preferences?.["dsp-default-preset-id"];
        if (defaultPresetId && presets[defaultPresetId]) {
            select.value = defaultPresetId;
            const activePresetLabel = document.getElementById("dsp-current-preset");
            if (activePresetLabel && select.options[select.selectedIndex]) {
                activePresetLabel.textContent = `${select.options[select.selectedIndex].text} (Default)`;
            }
        }
    }

    // Restore DSP active state
    const savedDSP = (window.serverState?.preferences?.["dsp-active-state"]);
    if (savedDSP) {
        try {
            const dsp = JSON.parse(savedDSP);
            dspEnabled = dsp.dspEnabled ?? true;

            const setDSPVal = (id, checked, value) => {
                const cb = document.getElementById(`dsp-cb-${id}`);
                const slider = document.getElementById(`dsp-slider-${id}`);
                if (cb && checked !== undefined) cb.checked = checked;
                if (slider && value !== undefined) slider.value = value;
            };

            setDSPVal("stereo", dsp.cb_stereo, dsp.stereo);
            setDSPVal("preamp", dsp.cb_preamp, dsp.preamp);
            setDSPVal("crossfeed", dsp.cb_crossfeed, dsp.crossfeed);
            setDSPVal("eq", dsp.cb_eq, dsp.eq);
            setDSPVal("bass", dsp.cb_bass, dsp.bass);
            setDSPVal("compressor", dsp.cb_compressor, dsp.compressor);
            setDSPVal("warmth", dsp.cb_warmth, dsp.warmth);
            setDSPVal("reverb", dsp.cb_reverb, dsp.reverb);
            setDSPVal("vocals", dsp.cb_vocals, dsp.vocals);
            setDSPVal("air", dsp.cb_air, dsp.air);

            const cbLimiter = document.getElementById("dsp-cb-limiter");
            if (cbLimiter && dsp.cb_limiter !== undefined) cbLimiter.checked = dsp.cb_limiter;

            // Trigger UI label refresh
            const bassVal = parseInt(document.getElementById("dsp-slider-bass")?.value ?? 0);
            const eqVal = parseInt(document.getElementById("dsp-slider-eq")?.value ?? 0);
            const vocalsVal = parseInt(document.getElementById("dsp-slider-vocals")?.value ?? 0);
            const airVal = parseInt(document.getElementById("dsp-slider-air")?.value ?? 0);
            const warmthVal = parseInt(document.getElementById("dsp-slider-warmth")?.value ?? 40);
            const stereoVal = parseInt(document.getElementById("dsp-slider-stereo")?.value ?? 100);
            const preampVal = parseFloat(document.getElementById("dsp-slider-preamp")?.value ?? 0);

            const valPreamp = document.getElementById("dsp-val-preamp");
            if (valPreamp) valPreamp.textContent = `${preampVal > 0 ? '+' : ''}${preampVal} dB`;

            const valBass = document.getElementById("dsp-val-bass");
            if (valBass) valBass.textContent = `${bassVal > 0 ? '+' : ''}${bassVal} dB`;
            const valEq = document.getElementById("dsp-val-eq");
            if (valEq) valEq.textContent = `${eqVal > 0 ? '+' : ''}${eqVal} dB`;
            const valVocals = document.getElementById("dsp-val-vocals");
            if (valVocals) valVocals.textContent = `${vocalsVal > 0 ? '+' : ''}${vocalsVal} dB`;
            const valAir = document.getElementById("dsp-val-air");
            if (valAir) valAir.textContent = `${airVal > 0 ? '+' : ''}${airVal} dB`;
            const valWarmth = document.getElementById("dsp-val-warmth");
            if (valWarmth) valWarmth.textContent = `${warmthVal}%`;
            const valStereo = document.getElementById("dsp-val-stereo");
            if (valStereo) valStereo.textContent = `${stereoVal}%`;

        } catch (e) {
            console.error("Error restoring DSP state:", e);
        }
    }

    // Restore DSP bypass state
    const savedBypass = window.serverState?.preferences?.["dsp-bypass"];
    const dspBypassVal = (savedBypass === true || savedBypass === "true");
    updateDSPBypassUI(dspBypassVal);

    // Hydrate saved favorites
    const savedFavs = (window.serverState?.preferences?.["player-selected-tracks"]);
    if (savedFavs) {
        try {
            const arr = JSON.parse(savedFavs);
            state.selectedTrackIds = new Set(arr);
        } catch (e) { }
    }

    const savedShuffle = (window.serverState?.preferences?.["player-shuffle-mode"]);
    if (savedShuffle === "true" || savedShuffle === true) {
        state.shuffleMode = "normal";
        state.lastShuffleMode = "normal";
    } else if (savedShuffle === "false" || savedShuffle === false) {
        state.shuffleMode = false;
        state.lastShuffleMode = (window.serverState?.preferences?.["player-last-shuffle-mode"]) || "normal";
    } else if (savedShuffle) {
        state.shuffleMode = savedShuffle;
        state.lastShuffleMode = savedShuffle;
    } else {
        state.lastShuffleMode = (window.serverState?.preferences?.["player-last-shuffle-mode"]) || "normal";
    }
    syncShuffleModeUI();

    const savedRepeat = (window.serverState?.preferences?.["player-repeat-mode"]);
    if (savedRepeat !== undefined && savedRepeat !== null) {
        state.repeatMode = savedRepeat;
        syncRepeatModeUI();
    }

    // Hydrate theme mode using the multi-theme system
    const savedThemeId = (window.serverState?.preferences?.["player-theme-id"]) || "system";
    applyTheme(savedThemeId);

    // Restore Layout Mode (right vs bottom panel layout)
    const savedLayoutMode = (window.serverState?.preferences?.["player-layout-mode"]) || "right";
    const appContainer = document.querySelector(".app-container");
    if (appContainer) {
        if (savedLayoutMode === "bottom") {
            appContainer.classList.add("layout-bottom");
        } else {
            appContainer.classList.remove("layout-bottom");
        }
    }

    // Restore Queue and Track state
    const savedQueue = (window.serverState?.preferences?.["player-queue"]);
    const savedTrackId = (window.serverState?.preferences?.["player-last-track-id"]);

    if (backendStatus && backendStatus.track_id) {
        // Sync directly with the active backend player state (e.g. controlled by remote)
        state.queueVersion = backendStatus.queue_version || 0;
        state.shuffleMode = backendStatus.shuffle_mode || false;
        state.repeatMode = backendStatus.repeat_mode || "none";
        syncRepeatModeUI();
        syncShuffleModeUI();

        // Fetch the queue active on the backend
        fetch("/api/player/queue")
            .then(res => res.json())
            .then(qData => {
                state.activePlaylist = qData.queue || [];
                generateShuffleIndices();
                updateQueueWidget();
                selectTrack(Number(backendStatus.track_id), false);
            })
            .catch(err => console.error("Error fetching active queue from backend:", err));
    } else {
        // Fallback: Restore from last saved desktop player preferences
        if (savedQueue) {
            try {
                state.activePlaylist = typeof savedQueue === "string" ? JSON.parse(savedQueue) : savedQueue;
                generateShuffleIndices();
                updateQueueWidget();

                if (savedTrackId) {
                    selectTrack(Number(savedTrackId), true);
                    // POST the queue to backend on startup to initialize it
                    fetch("/api/player/queue", {
                        method: "POST",
                        headers: { "Content-Type": "application/json" },
                        body: JSON.stringify({
                            queue: state.activePlaylist.map(tr => tr.id),
                            start_track_id: Number(savedTrackId)
                        })
                    }).then(res => res.json()).then(data => {
                        // Query status to get initial queue version
                        fetch("/api/player/status").then(r => r.json()).then(st => {
                            state.queueVersion = st.queue_version || 0;
                        }).catch(() => { });
                    }).catch(() => { });
                }
            } catch (e) {
                console.error("Error restoring queue/track state from prefs:", e);
            }
        }
    }

    // Hydrate 10-band graphic EQ custom presets & band sliders from window.serverState
    loadCustomEqPresets();
}


// Setup Resizers drag binds
function setupDragResizers() {
    const resizerLeft = document.getElementById("resizer-left");
    const sidebar = document.getElementById("app-sidebar");

    if (resizerLeft && sidebar) {
        resizerLeft.addEventListener("pointerdown", (e) => {
            e.preventDefault();
            document.body.classList.add("resizing-active");
            resizerLeft.classList.add("resizing");

            function onPointerMove(ev) {
                const newWidth = Math.max(160, Math.min(400, ev.clientX));
                sidebar.style.width = `${newWidth}px`;
            }
            function onPointerUp(ev) {
                document.body.classList.remove("resizing-active");
                resizerLeft.classList.remove("resizing");
                saveServerState("layout-sidebar-width", sidebar.offsetWidth);
                window.removeEventListener("pointermove", onPointerMove);
                window.removeEventListener("pointerup", onPointerUp);
            }

            window.addEventListener("pointermove", onPointerMove);
            window.addEventListener("pointerup", onPointerUp);
        });

        const savedLeftWidth = (window.serverState?.preferences?.["layout-sidebar-width"]);
        if (savedLeftWidth) {
            sidebar.style.width = `${savedLeftWidth}px`;
        }
    }

    const resizerRight = document.getElementById("resizer-right");
    if (resizerRight && detailsDrawer) {
        resizerRight.addEventListener("pointerdown", (e) => {
            e.preventDefault();
            document.body.classList.add("resizing-active");
            resizerRight.classList.add("resizing");

            function onPointerMove(ev) {
                const appContainer = document.querySelector(".app-container");
                if (appContainer && appContainer.classList.contains("layout-bottom")) {
                    const newHeight = Math.max(200, Math.min(600, window.innerHeight - ev.clientY));
                    detailsDrawer.style.height = `${newHeight}px`;
                } else {
                    const maxAllowedWidth = Math.floor(window.innerWidth * 0.5);
                    const newWidth = Math.max(380, Math.min(maxAllowedWidth, window.innerWidth - ev.clientX));
                    detailsDrawer.style.width = `${newWidth}px`;
                }
            }
            function onPointerUp(ev) {
                document.body.classList.remove("resizing-active");
                resizerRight.classList.remove("resizing");
                const appContainer = document.querySelector(".app-container");
                if (appContainer && appContainer.classList.contains("layout-bottom")) {
                    saveServerState("player-panel-height", detailsDrawer.offsetHeight);
                } else {
                    saveServerState("player-panel-width", detailsDrawer.offsetWidth);
                }
                window.removeEventListener("pointermove", onPointerMove);
                window.removeEventListener("pointerup", onPointerUp);
            }

            window.addEventListener("pointermove", onPointerMove);
            window.addEventListener("pointerup", onPointerUp);
        });

        const savedPanelWidth = (window.serverState?.preferences?.["player-panel-width"]) || "380";
        const savedPanelHeight = (window.serverState?.preferences?.["player-panel-height"]) || "320";
        const appContainer = document.querySelector(".app-container");
        if (appContainer && appContainer.classList.contains("layout-bottom")) {
            detailsDrawer.style.width = "100%";
            detailsDrawer.style.height = `${savedPanelHeight}px`;
        } else {
            detailsDrawer.style.height = "100%";
            detailsDrawer.style.width = `${savedPanelWidth}px`;
        }
    }

    const resizerHoriz = document.getElementById("resizer-horizontal");
    const topSection = document.getElementById("drawer-top-section");

    if (resizerHoriz && topSection) {
        resizerHoriz.addEventListener("pointerdown", (e) => {
            e.preventDefault();
            document.body.classList.add("resizing-active");

            function onPointerMove(ev) {
                const drawerRect = detailsDrawer.getBoundingClientRect();
                const relativeY = ev.clientY - drawerRect.top;
                const newHeight = Math.max(150, Math.min(500, relativeY));
                topSection.style.height = `${newHeight}px`;
            }
            function onPointerUp(ev) {
                document.body.classList.remove("resizing-active");
                saveServerState("player-drawer-top-height", topSection.offsetHeight);
                window.removeEventListener("pointermove", onPointerMove);
                window.removeEventListener("pointerup", onPointerUp);
            }

            window.addEventListener("pointermove", onPointerMove);
            window.addEventListener("pointerup", onPointerUp);
        });

        const savedTopHeight = (window.serverState?.preferences?.["player-drawer-top-height"]);
        if (savedTopHeight) {
            topSection.style.height = `${savedTopHeight}px`;
        }
    }
}

function updateSelectionUI() {
    const selectedArray = Array.from(state.selectedTrackIds);
    saveServerState("player-selected-tracks", JSON.stringify(selectedArray));

    const pbSelected = document.getElementById("pb-selected-count");
    if (pbSelected) {
        pbSelected.textContent = `Selected: ${state.selectedTrackIds.size}`;
    }
}

let smoothProgressInterval = null;
// function startSmoothProgress() {
//     if (smoothProgressInterval) clearInterval(smoothProgressInterval);
//     function updateProgress() {
//         if (!audio.duration) return;
//         const current = audio.currentTime;
//         const total = audio.duration;

//         const currentDurationStr = formatDuration(current);
//         if (audioTimeCurrent.textContent !== currentDurationStr) {
//             audioTimeCurrent.textContent = currentDurationStr;
//         }

//         const pctVal = ((current / total) * 100).toFixed(1);
//         if (audioSlider.value !== pctVal) {
//             audioSlider.value = pctVal;
//         }
//     }
//     smoothProgressInterval = setInterval(updateProgress, 250);
// }

function stopSmoothProgress() {
    if (smoothProgressInterval) {
        clearInterval(smoothProgressInterval);
        smoothProgressInterval = null;
    }
}



// Update Playback Queue Panel UI
function updateQueueWidget() {
    const queueList = document.getElementById("drawer-queue-list");
    if (!queueList) return;

    if (state.activePlaylist.length === 0) {
        queueList.innerHTML = `<div class="queue-empty-msg" style="padding: 14px; font-size: 12.5px; color: var(--text-low); text-align: center;"><i class="fa-solid fa-list-ul"></i> Queue is empty</div>`;
        return;
    }

    const count = state.activePlaylist.length;
    const currentIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(state.activeTrackId));

    let html = `
        <div class="queue-header-bar" style="display:flex; justify-content:space-between; align-items:center; padding: 4px 12px 12px 12px; border-bottom: 1px solid var(--border-glass); margin-bottom: 12px;">
            <span style="font-size: 11px; font-weight: 700; text-transform: uppercase; color: var(--text-low); letter-spacing: 0.8px;">Upcoming Queue (${count})</span>
            ${count > 1 ? '<button class="clear-queue-btn" onclick="clearQueue()"><i class="fa-solid fa-trash-can"></i> Clear</button>' : ''}
        </div>
    `;

    // Render currently playing track
    if (currentIdx !== -1) {
        const currentTrack = state.activePlaylist[currentIdx];
        html += `
            <div class="queue-item active-queue-item" onclick="playImmediate('${currentTrack.id}')" style="background: rgba(192, 132, 252, 0.12); border-left: 3px solid var(--accent-purple); padding: 8px 12px; margin-bottom: 4px; border-radius: 4px; display: flex; align-items: center; gap: 10px; cursor: pointer;">
                <div class="queue-track-number" style="color: var(--accent-purple); font-size: 11px;"><i class="fa-solid fa-play"></i></div>
                <div class="queue-track-details" style="display: flex; flex-direction: column; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex-grow: 1;">
                    <span class="queue-track-title" style="font-weight: 600; color: var(--text-high); font-size: 13px;">${escapeHtml(currentTrack.title)}</span>
                    <span class="queue-track-artist" style="color: var(--text-mid); font-size: 11px;">${escapeHtml(currentTrack.artist)}</span>
                </div>
            </div>
        `;

        // Find position in shuffle indices
        const orderIdx = state.shuffleIndices.indexOf(currentIdx);
        if (orderIdx !== -1) {
            let renderedCount = 0;
            for (let i = 1; i <= 9; i++) {
                let nextPos = orderIdx + i;
                if (nextPos >= count) {
                    if (state.repeatMode === "all") {
                        nextPos = nextPos % count;
                    } else {
                        break;
                    }
                }
                const nextTrackIdx = state.shuffleIndices[nextPos];
                const nextTrack = state.activePlaylist[nextTrackIdx];
                if (nextTrack) {
                    html += `
                        <div class="queue-item" onclick="playImmediate('${nextTrack.id}')" style="padding: 8px 12px; margin-bottom: 4px; border-radius: 4px; display: flex; align-items: center; gap: 10px; cursor: pointer; transition: background 0.2s;">
                            <div class="queue-track-number" style="color: var(--text-low); font-size: 11px; font-weight: 700; width: 14px;">${orderIdx + i + 1}</div>
                            <div class="queue-track-details" style="display: flex; flex-direction: column; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex-grow: 1;">
                                <span class="queue-track-title" style="font-weight: 500; color: var(--text-high); font-size: 13px;">${escapeHtml(nextTrack.title)}</span>
                                <span class="queue-track-artist" style="color: var(--text-mid); font-size: 11px;">${escapeHtml(nextTrack.artist)}</span>
                            </div>
                            <button class="queue-item-remove-btn" onclick="event.stopPropagation(); removeFromQueue('${nextTrack.id}')" title="Remove from Queue"><i class="fa-solid fa-xmark"></i></button>
                        </div>
                    `;
                    renderedCount++;
                }
            }
            if (renderedCount === 0 && count > 1) {
                html += `<div class="queue-end-msg" style="padding: 8px 14px; font-size: 11.5px; color: var(--text-low); text-align: center;"><i class="fa-solid fa-flag-checkered"></i> End of Playlist</div>`;
            }
        }
    } else {
        // If current track is not in the active filtered view, show the first 10 tracks of the queue
        for (let i = 0; i < Math.min(10, count); i++) {
            const trackIdx = state.shuffleIndices[i];
            const track = state.activePlaylist[trackIdx];
            if (track) {
                html += `
                    <div class="queue-item" onclick="playImmediate('${track.id}')" style="padding: 8px 12px; margin-bottom: 4px; border-radius: 4px; display: flex; align-items: center; gap: 10px; cursor: pointer; transition: background 0.2s;">
                        <div class="queue-track-number" style="color: var(--text-low); font-size: 11px; font-weight: 700; width: 14px;">${i + 1}</div>
                        <div class="queue-track-details" style="display: flex; flex-direction: column; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex-grow: 1;">
                            <span class="queue-track-title" style="font-weight: 500; color: var(--text-high); font-size: 13px;">${escapeHtml(track.title)}</span>
                            <span class="queue-track-artist" style="color: var(--text-mid); font-size: 11px;">${escapeHtml(track.artist)}</span>
                        </div>
                        <button class="queue-item-remove-btn" onclick="event.stopPropagation(); removeFromQueue('${track.id}')" title="Remove from Queue"><i class="fa-solid fa-xmark"></i></button>
                    </div>
                `;
            }
        }
    }

    queueList.innerHTML = html;
    syncRemoteQueue();
}

// Global Filter by Artist Helper
function filterByArtist(artistName) {
    const searchInput = document.getElementById("search-input");
    if (searchInput) {
        searchInput.value = artistName;
        state.searchQuery = artistName;
        state.currentPage = 1;

        // Show clear button
        const clearBtn = document.getElementById("search-clear-btn");
        if (clearBtn) clearBtn.style.display = "block";

        // Switch workspace back to library explorer
        const libNav = document.getElementById("btn-show-library");
        if (libNav) libNav.click();

        saveServerState("player-search-query", state.searchQuery);
        loadTracks();
    }
}

// Global Filter by Album Helper
function filterByAlbum(albumName) {
    const searchInput = document.getElementById("search-input");
    if (searchInput) {
        searchInput.value = albumName;
        state.searchQuery = albumName;
        state.currentPage = 1;

        // Show clear button
        const clearBtn = document.getElementById("search-clear-btn");
        if (clearBtn) clearBtn.style.display = "block";

        // Switch workspace back to library explorer
        const libNav = document.getElementById("btn-show-library");
        if (libNav) libNav.click();

        saveServerState("player-search-query", state.searchQuery);
        loadTracks();
    }
}

// Global Filter by Key Helper
function filterByKey(fullKeyName) {
    const parts = fullKeyName.split(" ");
    const note = parts[0];
    const scale = parts[1];

    const keyFilter = document.getElementById("filter-musical-key");
    const scaleFilter = document.getElementById("filter-major-minor");

    if (keyFilter) {
        keyFilter.value = note;
        state.keyFilter = note;
        saveServerState("player-filter-keyFilter", note);
    }
    if (scaleFilter && scale) {
        scaleFilter.value = scale;
        state.scaleFilter = scale;
        saveServerState("player-filter-scaleFilter", scale);
    }
    state.currentPage = 1;
    if (typeof window.updateAdvancedFiltersBadge === "function") {
        window.updateAdvancedFiltersBadge();
    }

    const libNav = document.getElementById("btn-show-library");
    if (libNav) libNav.click();

    loadTracks();
}

// Jump to the currently playing track in the Library Explorer
// (called when user clicks the album art in the Details Drawer)
async function jumpToTrackInExplorer(trackId) {
    // Find the track's position in the full active playlist to derive the page
    const idx = state.activePlaylist.findIndex(t => Number(t.id) === Number(trackId));

    // Clear any active filters so the track is visible
    if (idx === -1) {
        // Track not in current filtered playlist – clear search and reload
        state.searchQuery = "";
        saveServerState("player-search-query", "");
        const si = document.getElementById("search-input");
        if (si) si.value = "";
        state.currentPage = 1;
        updateAdvancedFiltersBadge();
    } else {
        const targetPage = Math.floor(idx / state.limit) + 1;
        state.currentPage = targetPage;
    }

    // Switch to Library workspace
    const libNav = document.getElementById("btn-show-library");
    if (libNav) libNav.click();

    // Load the page, then scroll to the active row
    await loadTracks();

    // Small delay to let DOM render
    setTimeout(() => {
        const activeRow = document.querySelector("#workspace-library .active-row");
        if (activeRow) {
            const container = activeRow.closest(".table-container") || activeRow.closest(".workspace-content");
            if (container) {
                const containerRect = container.getBoundingClientRect();
                const elRect = activeRow.getBoundingClientRect();
                const scrollTarget = container.scrollTop + (elRect.top - containerRect.top) - (containerRect.height / 2) + (elRect.height / 2);
                container.scrollTo({ top: scrollTarget, behavior: "smooth" });
            } else {
                activeRow.scrollIntoView({ behavior: "smooth", block: "center" });
            }
            // Brief flash highlight to draw the eye
            activeRow.style.transition = "background 0.1s";
            activeRow.style.background = "rgba(192, 132, 252, 0.35)";
            setTimeout(() => {
                activeRow.style.background = "";
                setTimeout(() => { activeRow.style.transition = ""; }, 400);
            }, 350);
        }
    }, 250);
}


// Global Filter by Emotion Helper
function filterByEmotion(emotionName) {
    const emotionFilter = document.getElementById("filter-emotion");
    if (emotionFilter) {
        emotionFilter.value = emotionName;
        state.emotionFilter = emotionName;
    }
    state.currentPage = 1;

    const libNav = document.getElementById("btn-show-library");
    if (libNav) libNav.click();

    loadTracks();
}

// Render Playback Queue workspace table
async function loadQueueWorkspace() {
    const tbody = document.getElementById("queue-workspace-tbody");
    const badge = document.getElementById("queue-status-badge");
    if (!tbody) return;

    if (!state.activePlaylist || state.activePlaylist.length === 0) {
        try {
            const res = await fetch("/api/player/queue");
            if (res.ok) {
                const qData = await res.json();
                if (qData.tracks && qData.tracks.length > 0) {
                    state.activePlaylist = qData.tracks;
                    generateShuffleIndices();
                } else {
                    const tRes = await fetch("/api/tracks?limit=50");
                    if (tRes.ok) {
                        const tData = await tRes.json();
                        state.activePlaylist = tData.tracks || [];
                        generateShuffleIndices();
                    }
                }
            }
        } catch (err) {
            console.error("loadQueueWorkspace fetch error:", err);
        }
    }

    const count = state.activePlaylist.length;
    if (badge) badge.textContent = `${count} Track${count === 1 ? '' : 's'}`;

    if (count === 0) {
        tbody.innerHTML = `<tr><td colspan="6" class="table-empty">Playback queue is empty. Go back to Library Explorer to add tracks.</td></tr>`;
        return;
    }

    let html = "";
    for (let i = 0; i < count; i++) {
        // Resolve index based on shuffle mode
        const playlistIdx = state.shuffleMode ? state.shuffleIndices[i] : i;
        const t = state.activePlaylist[playlistIdx];
        if (!t) continue;

        const isPlaying = String(t.id) === String(state.activeTrackId);
        const rowClass = isPlaying ? "active-row" : "";
        const playIcon = isPlaying ? '<i class="fa-solid fa-volume-high text-accent"></i>' : (i + 1);

        html += `
            <tr class="${rowClass}" data-id="${t.id}" style="cursor:pointer;">
                <td style="text-align:center; font-weight:700;">${playIcon}</td>
                <td style="font-weight:700; color:var(--text-high);" onclick="playImmediate('${t.id}')">${escapeHtml(t.title)}</td>
                <td onclick="event.stopPropagation()">${renderArtistLinks(t.artist)}</td>
                <td onclick="event.stopPropagation(); filterByAlbum(decodeURIComponent('${escapeJsParam(t.album)}'))" style="cursor: pointer; color: var(--accent-purple); text-decoration: underline; text-underline-offset: 3px; font-weight: 500;">${escapeHtml(t.album)}</td>
                <td class="col-center">${formatDuration(t.duration)}</td>
                <td class="col-action" onclick="event.stopPropagation()">
                    <button class="row-play-btn" onclick="playImmediate('${t.id}')" title="Play Now"><i class="fa-solid fa-play"></i></button>
                    <button class="row-play-btn" onclick="removeFromQueue('${t.id}')" style="margin-left: 4px; background: rgba(239, 68, 68, 0.12); border-color: rgba(239, 68, 68, 0.25); color: #f87171;" title="Remove from Queue"><i class="fa-solid fa-trash-can"></i></button>
                </td>
            </tr>
        `;
    }
    tbody.innerHTML = html;
}

// Sync active playlist queue to backend player endpoint
function syncQueueToServer() {
    const queueIds = state.activePlaylist.map(t => Number(t.id));
    fetch("/api/player/queue", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ track_ids: queueIds })
    }).catch(err => console.error("syncQueueToServer error:", err));
}

// Remove single track from active playlist queue
function removeFromQueue(trackId) {
    const idx = state.activePlaylist.findIndex(t => String(t.id) === String(trackId));
    if (idx !== -1) {
        state.activePlaylist.splice(idx, 1);
        generateShuffleIndices();
        updateQueueWidget();
        loadQueueWorkspace();
        syncQueueToServer();
    }
}

// Media Session Initialization Helpers

function setupMediaKeysAndKeyboard() {
    // Media Session API handlers (registers keyboard hardware keys and notification drawer buttons)
    if ('mediaSession' in navigator) {
        navigator.mediaSession.setActionHandler('play', () => {
            playAudio();
        });
        navigator.mediaSession.setActionHandler('pause', () => {
            pauseAudio();
        });
        navigator.mediaSession.setActionHandler('previoustrack', () => {
            playPreviousTrack();
        });
        navigator.mediaSession.setActionHandler('nexttrack', () => {
            playNextTrack();
        });
    }
}

function setAudioVolume(vol) {
    const safeVol = Math.max(0, Math.min(100, Math.round(vol)));
    state.volume = safeVol;

    // 1. Update WASAPI Music Volume Sliders (Green 🎵)
    const volSlider = document.getElementById("audio-volume-slider");
    const miniVolSlider = document.getElementById("mini-audio-volume-slider");
    if (volSlider) {
        volSlider.value = safeVol;
        volSlider.style.setProperty("--progress", `${safeVol}%`);
    }
    if (miniVolSlider) {
        miniVolSlider.value = safeVol;
    }

    // 2. Update volume icon
    if (typeof updateVolumeIcon === "function") {
        updateVolumeIcon(safeVol / 100);
    }

    // 3. Send volume to backend WASAPI music player API ONLY
    if (isWsConnected && ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ type: "command", command: `volume:${safeVol}` }));
    } else {
        fetch("/api/player/volume", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ volume: safeVol })
        }).catch(err => console.error("Music volume set error:", err));
    }
}

function showWasapiVolumeHud(text, level) {
    let hud = document.getElementById("wasapi-vol-hud");
    if (!hud) {
        hud = document.createElement("div");
        hud.id = "wasapi-vol-hud";
        hud.style.cssText = `
            position: fixed;
            bottom: 90px;
            right: 28px;
            z-index: 999999;
            background: rgba(14, 25, 23, 0.94);
            border: 1px solid rgba(121, 229, 207, 0.45);
            box-shadow: 0 12px 32px rgba(0, 0, 0, 0.65), 0 0 24px rgba(15, 118, 110, 0.35);
            backdrop-filter: blur(12px);
            -webkit-backdrop-filter: blur(12px);
            border-radius: 12px;
            padding: 10px 18px;
            color: #e7f5f1;
            font-family: system-ui, -apple-system, sans-serif;
            font-size: 0.9rem;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 12px;
            pointer-events: none;
            transition: opacity 0.2s ease, transform 0.2s ease;
            opacity: 0;
            transform: translateY(10px);
        `;
        document.body.appendChild(hud);
    }

    const icon = level === 0 ? "fa-volume-xmark" : (level < 50 ? "fa-volume-low" : "fa-volume-high");
    hud.innerHTML = `<i class="fa-solid ${icon}" style="color: #79e5cf; font-size: 1.1rem;"></i> <span>${escapeHtml(text)}</span>`;
    hud.style.opacity = "1";
    hud.style.transform = "translateY(0)";

    if (hud._timer) clearTimeout(hud._timer);
    hud._timer = setTimeout(() => {
        hud.style.opacity = "0";
        hud.style.transform = "translateY(10px)";
    }, 1500);
}

function setHardwareDriverVolume(vol) {
    const safeVol = Math.max(0, Math.min(100, Math.round(vol)));
    
    // Update System Driver Level Volume Sliders (Blue 🖥️)
    const sysSlider = document.getElementById("system-volume-slider");
    const miniSysSlider = document.getElementById("mini-system-volume-slider");
    const overlaySysSlider = document.getElementById("overlay-system-volume-slider");
    const sysValSpan = document.getElementById("system-volume-val");
    const overlaySysLabel = document.getElementById("overlay-system-volume-label");

    if (sysSlider) {
        sysSlider.value = safeVol;
        sysSlider.style.setProperty("--sys-progress", `${safeVol}%`);
    }
    if (miniSysSlider) {
        miniSysSlider.value = safeVol;
        miniSysSlider.style.setProperty("--sys-progress", `${safeVol}%`);
    }
    if (overlaySysSlider) {
        overlaySysSlider.value = safeVol;
        overlaySysSlider.style.setProperty("--sys-progress", `${safeVol}%`);
    }
    if (sysValSpan) sysValSpan.textContent = `${safeVol}%`;
    if (overlaySysLabel) overlaySysLabel.textContent = `${safeVol}%`;

    // Send driver level volume change to backend System Driver API ONLY
    if (isWsConnected && ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ type: "command", command: `system_volume:${safeVol}` }));
    } else {
        fetch("/api/system/volume", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ volume: safeVol })
        }).catch(err => console.error("Driver volume set error:", err));
    }
}

// Global Keyboard Shortcuts (Capture Phase): Navigation, Separate Music & Driver Volume, Play/Pause
document.addEventListener("keydown", (e) => {
    const active = document.activeElement;
    if (active && (active.tagName === "INPUT" || active.tagName === "TEXTAREA" || active.tagName === "SELECT" || active.isContentEditable)) {
        return;
    }

    const key = e.key ? e.key.toLowerCase() : "";
    const code = e.code ? e.code : "";
    const hasSysVolCombo = (e.ctrlKey && e.altKey) || (e.ctrlKey && e.shiftKey) || (e.altKey && e.shiftKey);

    const volStep = Number(state.volumeStepSize) || 2;

    // 1. System Hardware Driver Level Volume (Ctrl+Alt, Ctrl+Shift, Alt+Shift + Up/Down/M)
    if (hasSysVolCombo) {
        if (code === "ArrowUp" || key === "arrowup") {
            e.preventDefault();
            const sysSlider = document.getElementById("system-volume-slider");
            const curVol = sysSlider ? Number(sysSlider.value) : 80;
            const newVol = Math.min(100, curVol + volStep);
            setHardwareDriverVolume(newVol);
            showWasapiVolumeHud(`System Driver Volume: ${newVol}%`, newVol);
            return;
        }
        if (code === "ArrowDown" || key === "arrowdown") {
            e.preventDefault();
            const sysSlider = document.getElementById("system-volume-slider");
            const curVol = sysSlider ? Number(sysSlider.value) : 80;
            const newVol = Math.max(0, curVol - volStep);
            setHardwareDriverVolume(newVol);
            showWasapiVolumeHud(`System Driver Volume: ${newVol}%`, newVol);
            return;
        }
        if (code === "KeyM" || key === "m") {
            e.preventDefault();
            if (e.repeat) return; // Ignore hold repeat for mute toggle
            const sysSlider = document.getElementById("system-volume-slider");
            const curVol = sysSlider ? Number(sysSlider.value) : 80;
            if (curVol > 0) {
                state._savedPreMuteSysVol = curVol;
                setHardwareDriverVolume(0);
                showWasapiVolumeHud("System Driver Volume: Muted (0%)", 0);
            } else {
                const restoreVol = state._savedPreMuteSysVol || 80;
                setHardwareDriverVolume(restoreVol);
                showWasapiVolumeHud(`System Driver Volume: Unmuted (${restoreVol}%)`, restoreVol);
            }
            return;
        }
    }

    // 2. Music Player Volume (Up Arrow / Down Arrow / M without modifiers)
    if (code === "ArrowUp" || key === "arrowup") {
        e.preventDefault();
        const curVol = Number(state.volume) || 0;
        const newVol = Math.min(100, curVol + volStep);
        setAudioVolume(newVol);
        showWasapiVolumeHud(`Music Player Volume: ${newVol}%`, newVol);
        return;
    }

    if (code === "ArrowDown" || key === "arrowdown") {
        e.preventDefault();
        const curVol = Number(state.volume) || 0;
        const newVol = Math.max(0, curVol - volStep);
        setAudioVolume(newVol);
        showWasapiVolumeHud(`Music Player Volume: ${newVol}%`, newVol);
        return;
    }

    if (code === "KeyM" || key === "m") {
        e.preventDefault();
        const curVol = Number(state.volume) || 0;
        if (curVol > 0) {
            state._savedPreMuteVol = curVol;
            setAudioVolume(0);
            showWasapiVolumeHud("Music Player Volume: Muted (0%)", 0);
        } else {
            const restoreVol = state._savedPreMuteVol || 80;
            setAudioVolume(restoreVol);
            showWasapiVolumeHud(`Music Player Volume: Unmuted (${restoreVol}%)`, restoreVol);
        }
        return;
    }

    // 4. Play / Pause (Space)
    if (code === "Space" || key === " " || key === "spacebar") {
        e.preventDefault();
        if (state.isPlaying) {
            pauseAudio();
        } else {
            playAudio();
        }
        return;
    }

    // 5. Prev / Next (Left / Right Arrow)
    if (code === "ArrowLeft" || key === "arrowleft") {
        e.preventDefault();
        playPreviousTrack();
        return;
    }

    if (code === "ArrowRight" || key === "arrowright") {
        e.preventDefault();
        playNextTrack();
        return;
    }
}, true);

function startRemoteCommandPolling() {
    setInterval(async () => {
        try {
            const res = await fetch("/api/remote/pop_command");
            if (res.ok) {
                const data = await res.json();
                if (data.command) {
                    const cmd = data.command;
                    if (cmd === "play") {
                        playAudio();
                    } else if (cmd === "pause") {
                        pauseAudio();
                    } else if (cmd === "next") {
                        playNextTrack();
                    } else if (cmd === "prev") {
                        playPreviousTrack();
                    } else if (cmd === "shuffle") {
                        if (btnShuffle) btnShuffle.click();
                    } else if (cmd.startsWith("shuffle:")) {
                        const val = cmd.substring(8);
                        state.shuffleMode = (val === "false" ? false : (val === "true" ? true : val));
                        // Non-destructive album shuffle mode does not rebuild queue.
                        saveServerState("player-shuffle-mode", state.shuffleMode);
                        syncShuffleModeUI();
                        generateShuffleIndices();
                        fetch("/api/player/mode", {
                            method: "POST",
                            headers: { "Content-Type": "application/json" },
                            body: JSON.stringify({ shuffle_mode: state.shuffleMode })
                        }).catch(() => { });
                    } else if (cmd === "repeat") {
                        if (btnRepeat) btnRepeat.click();
                    } else if (cmd.startsWith("repeat:")) {
                        const val = cmd.substring(7);
                        state.repeatMode = val;
                        saveServerState("player-repeat-mode", state.repeatMode);
                        syncRepeatModeUI();
                        fetch("/api/player/mode", {
                            method: "POST",
                            headers: { "Content-Type": "application/json" },
                            body: JSON.stringify({ repeat_mode: state.repeatMode })
                        }).catch(() => { });
                    } else if (cmd.startsWith("play_album:")) {
                        const albumName = decodeURIComponent(cmd.substring(11));
                        playAlbumOnDesktop(albumName);
                    } else if (cmd.startsWith("shuffle_album:")) {
                        const albumName = decodeURIComponent(cmd.substring(14));
                        shuffleAlbumOnDesktop(albumName);
                    } else if (cmd.startsWith("play_track:")) {
                        const trackId = cmd.substring(11);
                        playImmediate(trackId);
                    } else if (cmd.startsWith("volume:")) {
                        const volVal = parseInt(cmd.substring(7));
                        if (!isNaN(volVal)) {
                            if (volumeSlider) volumeSlider.value = volVal;
                            if (audio) audio.volume = volVal / 100; // Safe check
                            updateVolumeIcon(volVal / 100);
                            if (volVal > 0) {
                                state.lastVolume = volVal;
                                saveServerState("player-last-volume", volVal);
                            }
                            saveServerState("player-volume", volVal);
                        }
                    } else if (cmd.startsWith("seek:")) {
                        const pctVal = parseFloat(cmd.substring(5));
                        if (!isNaN(pctVal) && state.duration) {
                            // FIX: Send API request to backend instead of manipulating audio element
                            fetch("/api/player/seek", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ time: (pctVal / 100) * state.duration }) });
                        }
                    } else if (cmd.startsWith("seek_seconds:")) {
                        const secVal = parseFloat(cmd.substring(13));
                        if (!isNaN(secVal)) {
                            // FIX: Send API request to backend
                            fetch("/api/player/seek", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ time: secVal }) });
                        }
                    }
                }
            }
        } catch (err) {
            console.error("Error polling remote command:", err);
        }
    }, 500);
}

function playAlbumOnDesktop(albumName) {
    // Switch workspace back to library explorer
    const libNav = document.getElementById("btn-show-library");
    if (libNav) {
        libNav.click();
    }

    // Reset filters cache to force recreation of activePlaylist queue
    lastSearchQuery = null;
    lastVocalFilter = null;
    lastCharacterFilter = null;
    lastSortBy = null;
    lastSortOrder = null;

    state.searchQuery = albumName;
    state.vocalFilter = "";
    state.characterFilter = "";
    state.sortBy = "title";
    state.sortOrder = "asc";
    saveServerState("player-sort-order", state.sortOrder);
    state.currentPage = 1;

    const searchInput = document.getElementById("search-input");
    if (searchInput) {
        searchInput.value = albumName;
    }
    const clearBtn = document.getElementById("search-clear-btn");
    if (clearBtn) {
        clearBtn.style.display = "block";
    }

    // Reset vocal filter chips in UI
    if (typeof filterTags !== "undefined") {
        filterTags.forEach(tag => {
            if (tag.getAttribute("data-vocal") === "") {
                tag.classList.add("active");
            } else {
                tag.classList.remove("active");
            }
        });
    }

    // Reset vibe filter dropdown in UI
    if (filterCharacter) {
        filterCharacter.value = "";
    }

    loadTracks().then(() => {
        if (state.activePlaylist.length > 0) {
            playImmediate(state.activePlaylist[0].id);
        }
    });
}

function shuffleAlbumOnDesktop(albumName) {
    const libNav = document.getElementById("btn-show-library");
    if (libNav) {
        libNav.click();
    }

    lastSearchQuery = null;
    lastVocalFilter = null;
    lastCharacterFilter = null;
    lastSortBy = null;
    lastSortOrder = null;

    state.searchQuery = albumName;
    state.vocalFilter = "";
    state.characterFilter = "";
    state.sortBy = "title";
    state.sortOrder = "asc";
    saveServerState("player-sort-order", state.sortOrder);
    state.currentPage = 1;

    const searchInput = document.getElementById("search-input");
    if (searchInput) {
        searchInput.value = albumName;
    }
    const clearBtn = document.getElementById("search-clear-btn");
    if (clearBtn) {
        clearBtn.style.display = "block";
    }

    if (typeof filterTags !== "undefined") {
        filterTags.forEach(tag => {
            if (tag.getAttribute("data-vocal") === "") {
                tag.classList.add("active");
            } else {
                tag.classList.remove("active");
            }
        });
    }

    if (filterCharacter) {
        filterCharacter.value = "";
    }

    loadTracks().then(() => {
        if (state.activePlaylist.length > 0) {
            state.activePlaylist.sort(() => Math.random() - 0.5);
            state.shuffleIndices = Array.from({ length: state.activePlaylist.length }, (_, i) => i);
            const trackIds = state.activePlaylist.map(t => Number(t.id));
            fetch("/api/player/queue", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ queue: trackIds, start_track_id: trackIds[0] })
            }).then(() => {
                playImmediate(trackIds[0]);
            });
        }
    });
}

lastSyncedQueueStr = "";
function syncRemoteQueue() {
    const queueTracks = state.activePlaylist.map(t => ({
        id: t.id,
        title: t.title,
        artist: t.artist,
        duration: t.duration
    }));

    const queueStr = JSON.stringify(queueTracks);
    if (queueStr === lastSyncedQueueStr) return;
    lastSyncedQueueStr = queueStr;

    // Persist queue in browser
    saveServerState("player-queue", JSON.stringify(state.activePlaylist));

    fetch('/api/remote/update_queue', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: queueStr
    }).catch(err => console.error("Error syncing remote queue:", err));
}

// Check viewport width and auto-collapse sidebars on narrow displays
function setupResponsiveLayout() {
    const container = document.querySelector(".app-container");
    if (!container) return;

    const checkResponsive = () => {
        if (window.innerWidth < 768) {
            if (!container.classList.contains("sidebar-collapsed")) {
                container.classList.add("sidebar-collapsed");
            }
        }
    };

    // Check initially
    checkResponsive();

    // Check on resize (throttled)
    let resizeTimeout;
    window.addEventListener("resize", () => {
        clearTimeout(resizeTimeout);
        resizeTimeout = setTimeout(checkResponsive, 150);
    });
}

function setupNavigation() {
    document.querySelectorAll("[data-workspace]").forEach(navItem => {
        navItem.addEventListener("click", (e) => {
            e.preventDefault();
            let targetWorkspaceId = navItem.getAttribute("data-workspace");
            if (!targetWorkspaceId) return;

            // Toggle back to workspace-library if clicking active workspace DSP
            if (navItem.id === "btn-drawer-dsp" && state.activeWorkspace === "workspace-dsp") {
                targetWorkspaceId = "workspace-library";
            }

            document.querySelectorAll(".nav-item").forEach(n => {
                if (n.getAttribute("data-workspace") === targetWorkspaceId) {
                    n.classList.add("active");
                } else {
                    n.classList.remove("active");
                }
            });

            const btnDsp = document.getElementById("btn-drawer-dsp");
            if (btnDsp) {
                if (targetWorkspaceId === "workspace-dsp") btnDsp.classList.add("active");
                else btnDsp.classList.remove("active");
            }

            document.querySelectorAll(".workspace-panel").forEach(p => p.classList.remove("active"));
            const targetPanel = document.getElementById(targetWorkspaceId);
            if (targetPanel) targetPanel.classList.add("active");

            state.activeWorkspace = targetWorkspaceId;
            if (typeof saveServerState === "function") saveServerState("player-active-workspace", targetWorkspaceId);

            // Auto-collapse sidebar on mobile screen navigations
            if (window.innerWidth < 768) {
                const container = document.querySelector(".app-container");
                if (container && !container.classList.contains("sidebar-collapsed")) {
                    container.classList.add("sidebar-collapsed");
                }
            }

            if (targetWorkspaceId === "workspace-settings") {
                loadAudioDevices();
                if (typeof fetchSystemVolume === "function") fetchSystemVolume();
            }

            if (targetWorkspaceId === "workspace-manage-library") {
                loadManageLibraryWorkspace();
            } else if (targetWorkspaceId === "workspace-albums") {
                loadAlbumsWorkspace();
            } else if (targetWorkspaceId === "workspace-favorites") {
                if (typeof loadFavoritesWorkspace === "function") loadFavoritesWorkspace();
            } else if (targetWorkspaceId === "workspace-themes") {
                if (typeof loadThemeExplorer === "function") loadThemeExplorer();
            } else if (targetWorkspaceId === "workspace-playlist-builder") {
                if (typeof loadPlaylistBuilderTracks === "function") loadPlaylistBuilderTracks();
            }
        });
    });
}


async function loadAlbumsWorkspace() {
    state.activeWorkspace = "workspace-albums";
    document.querySelectorAll(".workspace-panel").forEach(p => p.classList.remove("active"));
    document.getElementById("workspace-albums").classList.add("active");

    const grid = document.getElementById("albums-grid");
    grid.innerHTML = '<div style="color: var(--text-muted);">Loading albums...</div>';

    try {
        const res = await fetch("/api/albums");
        if (res.ok) {
            const albums = await res.json();
            grid.innerHTML = "";
            albums.forEach(a => {
                const count = a.trackCount || 1;
                const countStr = count === 1 ? "1 track" : `${count} tracks`;
                const card = document.createElement("div");
                card.className = "grid-card";
                card.innerHTML = `
                    <img src="/api/art?id=${a.trackId}" class="grid-card-img" onerror="handleArtError(this)">
                    <div class="grid-card-title" title="${escapeHtml(a.name)}">${escapeHtml(a.name)}</div>
                    <div class="grid-card-artist" title="${escapeHtml(a.artist)}">${escapeHtml(a.artist)} • ${countStr}</div>
                `;
                card.addEventListener("click", () => {
                    // Switch to library and filter by exact album
                    document.getElementById("btn-show-library").click();
                    state.searchQuery = a.name;
                    if (searchInput) searchInput.value = a.name;
                    loadTracks();
                });
                grid.appendChild(card);
            });
        }
    } catch (e) {
        grid.innerHTML = '<div style="color: red;">Error loading albums</div>';
        console.error("Error loading albums:", e);
    }
}

// Manage Library Workspace (Dedicated Songs & Albums Deletion)
let mlAllTracks = [];
let mlAllAlbums = [];
let mlActiveTab = "tracks";

async function loadManageLibraryWorkspace() {
    state.activeWorkspace = "workspace-manage-library";
    document.querySelectorAll(".workspace-panel").forEach(p => p.classList.remove("active"));
    const mlPanel = document.getElementById("workspace-manage-library");
    if (mlPanel) mlPanel.classList.add("active");

    // Init tab switching listeners if not bound
    const tabTracksBtn = document.getElementById("btn-ml-tab-tracks");
    const tabAlbumsBtn = document.getElementById("btn-ml-tab-albums");
    const secTracks = document.getElementById("ml-section-tracks");
    const secAlbums = document.getElementById("ml-section-albums");
    const inputTracksSearch = document.getElementById("ml-tracks-search");
    const inputAlbumsSearch = document.getElementById("ml-albums-search");

    if (tabTracksBtn && !tabTracksBtn.dataset.bound) {
        tabTracksBtn.dataset.bound = "true";
        tabTracksBtn.addEventListener("click", () => {
            mlActiveTab = "tracks";
            tabTracksBtn.classList.add("active");
            if (tabAlbumsBtn) tabAlbumsBtn.classList.remove("active");
            if (secTracks) secTracks.style.display = "flex";
            if (secAlbums) secAlbums.style.display = "none";
        });
    }
    if (tabAlbumsBtn && !tabAlbumsBtn.dataset.bound) {
        tabAlbumsBtn.dataset.bound = "true";
        tabAlbumsBtn.addEventListener("click", () => {
            mlActiveTab = "albums";
            tabAlbumsBtn.classList.add("active");
            if (tabTracksBtn) tabTracksBtn.classList.remove("active");
            if (secTracks) secTracks.style.display = "none";
            if (secAlbums) secAlbums.style.display = "flex";
            renderMlAlbums();
        });
    }

    if (inputTracksSearch && !inputTracksSearch.dataset.bound) {
        inputTracksSearch.dataset.bound = "true";
        inputTracksSearch.addEventListener("input", () => renderMlTracks());
    }
    if (inputAlbumsSearch && !inputAlbumsSearch.dataset.bound) {
        inputAlbumsSearch.dataset.bound = "true";
        inputAlbumsSearch.addEventListener("input", () => renderMlAlbums());
    }

    // Load Tracks
    const tbody = document.getElementById("ml-tracks-tbody");
    if (tbody) tbody.innerHTML = `<tr><td colspan="5" class="table-loading"><i class="fa-solid fa-spinner fa-spin"></i> Loading songs...</td></tr>`;

    try {
        const trRes = await fetch("/api/tracks?limit=10000");
        if (trRes.ok) {
            const data = await trRes.json();
            mlAllTracks = data.tracks || [];
            renderMlTracks();
        }
    } catch (e) {
        console.error("Error loading tracks for manage library:", e);
    }

    // Load Albums
    try {
        const alRes = await fetch("/api/albums");
        if (alRes.ok) {
            mlAllAlbums = await alRes.json();
            if (mlActiveTab === "albums") renderMlAlbums();
        }
    } catch (e) {
        console.error("Error loading albums for manage library:", e);
    }
}

function renderMlTracks() {
    const tbody = document.getElementById("ml-tracks-tbody");
    const badge = document.getElementById("ml-tracks-count-badge");
    const searchVal = (document.getElementById("ml-tracks-search")?.value || "").toLowerCase().trim();

    if (!tbody) return;

    const filtered = mlAllTracks.filter(t => 
        !searchVal || 
        (t.title && t.title.toLowerCase().includes(searchVal)) ||
        (t.artist && t.artist.toLowerCase().includes(searchVal)) ||
        (t.album && t.album.toLowerCase().includes(searchVal))
    );

    if (badge) badge.textContent = `${filtered.length} / ${mlAllTracks.length} Songs`;

    if (filtered.length === 0) {
        tbody.innerHTML = `<tr><td colspan="5" class="table-empty">No matching songs found.</td></tr>`;
        return;
    }

    let html = "";
    filtered.forEach(t => {
        html += `
            <tr>
                <td style="font-weight:700; color:var(--text-high);">${escapeHtml(t.title)}</td>
                <td>${escapeHtml(t.artist)}</td>
                <td style="color:var(--text-mid);">${escapeHtml(t.album)}</td>
                <td class="col-center">${formatDuration(t.duration)}</td>
                <td class="col-action manage-action-cell">
                    <button class="manage-delete-btn" title="Delete Song" onclick="openDeleteModal({ type: 'track', id: '${t.id}', title: decodeURIComponent('${escapeJsParam(t.title)}') })">
                        <i class="fa-solid fa-trash-can"></i> Delete
                    </button>
                </td>
            </tr>
        `;
    });
    tbody.innerHTML = html;
}

function renderMlAlbums() {
    const grid = document.getElementById("ml-albums-grid");
    const badge = document.getElementById("ml-albums-count-badge");
    const searchVal = (document.getElementById("ml-albums-search")?.value || "").toLowerCase().trim();

    if (!grid) return;

    const filtered = mlAllAlbums.filter(a =>
        !searchVal ||
        (a.name && a.name.toLowerCase().includes(searchVal)) ||
        (a.artist && a.artist.toLowerCase().includes(searchVal))
    );

    if (badge) badge.textContent = `${filtered.length} / ${mlAllAlbums.length} Albums`;

    if (filtered.length === 0) {
        grid.innerHTML = `<div style="color: var(--text-muted); grid-column: 1 / -1; padding: 20px;">No matching albums found.</div>`;
        return;
    }

    grid.innerHTML = "";
    filtered.forEach(a => {
        const count = a.trackCount || 1;
        const countStr = count === 1 ? "1 track" : `${count} tracks`;
        const card = document.createElement("div");
        card.className = "grid-card";
        card.style.position = "relative";
        card.innerHTML = `
            <img src="/api/art?id=${a.trackId}" class="grid-card-img" onerror="handleArtError(this)">
            <div class="grid-card-title" title="${escapeHtml(a.name)}">${escapeHtml(a.name)}</div>
            <div class="grid-card-artist" title="${escapeHtml(a.artist)}">${escapeHtml(a.artist)} • ${countStr}</div>
            <button class="btn-delete-album-ml" title="Delete Album" style="margin-top: 10px; width: 100%; background: rgba(239, 68, 68, 0.2); border: 1px solid rgba(239, 68, 68, 0.4); color: #f87171; border-radius: 6px; padding: 6px; display: flex; align-items: center; justify-content: center; gap: 6px; font-weight: 600; cursor: pointer; font-size: 12px; backdrop-filter: blur(4px);">
                <i class="fa-solid fa-trash-can"></i> Delete Album
            </button>
        `;
        const delBtn = card.querySelector(".btn-delete-album-ml");
        if (delBtn) {
            delBtn.addEventListener("click", (e) => {
                e.stopPropagation();
                openDeleteModal({ type: 'album', name: a.name, count: a.trackCount });
            });
        }
        grid.appendChild(card);
    });
}

// Global Delete Modal State and Functions
let pendingDeleteTarget = null;

function openDeleteModal(target) {
    pendingDeleteTarget = target;
    const modal = document.getElementById("delete-confirm-modal");
    const titleEl = document.getElementById("delete-modal-title");
    const msgEl = document.getElementById("delete-modal-msg");
    const chkEl = document.getElementById("delete-modal-file-chk");
    
    if (!modal) return;
    
    if (chkEl) chkEl.checked = true; // default checked
    
    if (target.type === "track") {
        if (titleEl) titleEl.textContent = "Delete Song";
        if (msgEl) msgEl.innerHTML = `Are you sure you want to remove <strong style="color:#f8fafc;">${escapeHtml(target.title || "this track")}</strong> from your workstation library?`;
    } else if (target.type === "album") {
        if (titleEl) titleEl.textContent = "Delete Album";
        if (msgEl) msgEl.innerHTML = `Are you sure you want to remove the album <strong style="color:#f8fafc;">${escapeHtml(target.name || "this album")}</strong> (${target.count || 'all'} tracks) from your workstation library?`;
    }
    
    modal.style.display = "flex";
}

function closeDeleteModal() {
    pendingDeleteTarget = null;
    const modal = document.getElementById("delete-confirm-modal");
    if (modal) modal.style.display = "none";
}

async function confirmDeleteAction() {
    if (!pendingDeleteTarget) return;

    const chkEl = document.getElementById("delete-modal-file-chk");
    const deleteFile = chkEl ? chkEl.checked : true;
    
    const target = pendingDeleteTarget;
    closeDeleteModal();
    
    try {
        let payload = { delete_file: deleteFile };
        if (target.type === "track") {
            payload.track_id = target.id;
        } else if (target.type === "album") {
            payload.album = target.name;
            payload.album_name = target.name;
        }
        
        const res = await fetch("/api/tracks/delete", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
        
        let result = {};
        try {
            result = await res.json();
        } catch (jsonErr) {
            result = { error: `Server returned non-JSON response (HTTP ${res.status})` };
        }

        if (res.ok && (result.success || result.status === "success")) {
            if (target.type === "track") {
                if (Number(target.id) === Number(state.activeTrackId)) {
                    fetch("/api/player/stop", { method: "POST" }).catch(() => {});
                }
                loadTracks();
                if (state.activeWorkspace === "workspace-manage-library") {
                    loadManageLibraryWorkspace();
                }
            } else if (target.type === "album") {
                loadAlbumsWorkspace();
                loadTracks();
                if (state.activeWorkspace === "workspace-manage-library") {
                    loadManageLibraryWorkspace();
                }
            }
        } else {
            alert(`Error deleting item: ${result.error || 'Unknown error'}`);
        }
    } catch (err) {
        console.error("Delete error:", err);
        alert(`Failed to complete delete request: ${err.message}`);
    }
}

// Bind Delete Modal Event Listeners
document.addEventListener("DOMContentLoaded", () => {
    const btnCancel = document.getElementById("btn-delete-cancel");
    const btnConfirm = document.getElementById("btn-delete-confirm");
    const modalOverlay = document.getElementById("delete-confirm-modal");

    if (btnCancel) {
        btnCancel.addEventListener("click", closeDeleteModal);
    }
    if (btnConfirm) {
        btnConfirm.addEventListener("click", confirmDeleteAction);
    }
    if (modalOverlay) {
        modalOverlay.addEventListener("click", (e) => {
            if (e.target === modalOverlay) closeDeleteModal();
        });
    }
});

// Flush state on page hide/unload
window.addEventListener("pagehide", () => {
    const keys = Object.keys(userStateBuffer);
    if (keys.length === 0) return;
    const payload = { ...userStateBuffer };
    userStateBuffer = {};
    try {
        // Use keepalive fetch to ensure request finishes even if page closes
        fetch("/api/user_state", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
            keepalive: true
        });
    } catch (e) { }
});

// Render Acoustic & Production Profiler
function renderProfilerGrid(track) {
    const grid = document.getElementById("profiler-grid");
    if (!grid) return;

    const getVal = (v) => (v !== undefined && v !== null ? parseFloat(v) : 0.0);

    const bpm = getVal(track.bpm);
    const dr = getVal(track.dynamic_range);
    const ld = getVal(track.volume_dynamics);
    const brightness = getVal(track.brightness);
    const stereoWidth = getVal(track.stereo_width);
    const reverbVal = getVal(track.reverb_score);
    const reverb = reverbVal >= 0.99 ? (0.3 + getVal(track.depth_score) * 0.5 + getVal(track.stereo_width) * 0.2) : reverbVal;

    const acoustic = getVal(track.production_acoustic);
    const modern = getVal(track.production_modern);
    const vintage = getVal(track.production_analog);
    const cinematic = getVal(track.production_cinematic);

    // Left column: ACOUSTIC
    let colAcousticHtml = `
        <div>
            <div style="font-size: 9px; font-weight: 700; text-transform: uppercase; color: var(--text-low); letter-spacing: 0.8px; margin-bottom: 12px;">Acoustic</div>
            <div style="display:flex; flex-direction:column; gap:10px;">
                <!-- Tempo -->
                <div style="display:flex; justify-content:space-between; align-items:center; font-size:11px;">
                    <span style="display:flex; align-items:center; gap:6px; color: var(--text-mid);"><i class="fa-solid fa-gauge-simple-high" style="color:#22d3ee;"></i> Tempo</span>
                    <span style="font-weight:700; color:#22d3ee;">${bpm > 0 ? `${Math.round(bpm)} BPM` : 'N/A'}</span>
                </div>
                <!-- Dynamic Range -->
                <div style="display:flex; justify-content:space-between; align-items:center; font-size:11px;">
                    <span style="display:flex; align-items:center; gap:6px; color: var(--text-mid);"><i class="fa-solid fa-arrows-left-right" style="color:#fb7185;"></i> Dynamic Range</span>
                    <span style="font-weight:700; color:#fb7185;">${dr > 0 ? `${dr.toFixed(1)} dB` : 'N/A'}</span>
                </div>
                <!-- Loudness Range -->
                <div style="display:flex; justify-content:space-between; align-items:center; font-size:11px; margin-bottom:4px;">
                    <span style="display:flex; align-items:center; gap:6px; color: var(--text-mid);"><i class="fa-solid fa-chart-line" style="color:#fb7185;"></i> Loudness Range</span>
                    <span style="font-weight:700; color:#fb7185;">${ld > 0 ? `${ld.toFixed(1)} dB` : 'N/A'}</span>
                </div>
                <!-- Brightness -->
                <div>
                    <div style="display:flex; justify-content:space-between; font-size:10px; color: var(--text-low); margin-bottom:3px;">
                        <span>Brightness</span>
                        <span>${Math.round(brightness * 100)}%</span>
                    </div>
                    <div style="height:3px; background: var(--border-glass); border-radius:2px; overflow:hidden;">
                        <div style="height:100%; width:${Math.round(brightness * 100)}%; background:linear-gradient(90deg, var(--accent-rose), #fb7185); border-radius:2px;"></div>
                    </div>
                </div>
                <!-- Stereo Width -->
                <div>
                    <div style="display:flex; justify-content:space-between; font-size:10px; color: var(--text-low); margin-bottom:3px;">
                        <span>Stereo Width</span>
                        <span>${Math.round(stereoWidth * 100)}%</span>
                    </div>
                    <div style="height:3px; background: var(--border-glass); border-radius:2px; overflow:hidden;">
                        <div style="height:100%; width:${Math.min(100, Math.round(stereoWidth * 100))}%; background:linear-gradient(90deg, var(--accent-purple), #c084fc); border-radius:2px;"></div>
                    </div>
                </div>
                <!-- Reverb Space -->
                <div>
                    <div style="display:flex; justify-content:space-between; font-size:10px; color: var(--text-low); margin-bottom:3px;">
                        <span>Reverb/Space</span>
                        <span>${Math.round(reverb * 100)}%</span>
                    </div>
                    <div style="height:3px; background: var(--border-glass); border-radius:2px; overflow:hidden;">
                        <div style="height:100%; width:${Math.round(reverb * 100)}%; background:linear-gradient(90deg, #059669, #34d399); border-radius:2px;"></div>
                    </div>
                </div>
            </div>
        </div>
    `;

    // Right column: PRODUCTION STYLE
    let colProductionHtml = `
        <div>
            <div style="font-size: 9px; font-weight: 700; text-transform: uppercase; color: var(--text-low); letter-spacing: 0.8px; margin-bottom: 12px;">Production Style</div>
            <div style="display:flex; flex-direction:column; gap:10px;">
                <!-- Acoustic -->
                <div>
                    <div style="display:flex; justify-content:space-between; font-size:10px; color: var(--text-low); margin-bottom:3px;">
                        <span>Acoustic (Natural)</span>
                        <span>${Math.round(acoustic * 100)}%</span>
                    </div>
                    <div style="height:3px; background: var(--border-glass); border-radius:2px; overflow:hidden;">
                        <div style="height:100%; width:${Math.round(acoustic * 100)}%; background:linear-gradient(90deg, #059669, #34d399); border-radius:2px;"></div>
                    </div>
                </div>
                <!-- Modern -->
                <div>
                    <div style="display:flex; justify-content:space-between; font-size:10px; color: var(--text-low); margin-bottom:3px;">
                        <span>Modern (Electronic)</span>
                        <span>${Math.round(modern * 100)}%</span>
                    </div>
                    <div style="height:3px; background: var(--border-glass); border-radius:2px; overflow:hidden;">
                        <div style="height:100%; width:${Math.round(modern * 100)}%; background:linear-gradient(90deg, #06b6d4, #22d3ee); border-radius:2px;"></div>
                    </div>
                </div>
                <!-- Vintage -->
                <div>
                    <div style="display:flex; justify-content:space-between; font-size:10px; color: var(--text-low); margin-bottom:3px;">
                        <span>Vintage (Analog)</span>
                        <span>${Math.round(vintage * 100)}%</span>
                    </div>
                    <div style="height:3px; background: var(--border-glass); border-radius:2px; overflow:hidden;">
                        <div style="height:100%; width:${Math.round(vintage * 100)}%; background:linear-gradient(90deg, var(--accent-purple), #c084fc); border-radius:2px;"></div>
                    </div>
                </div>
                <!-- Cinematic -->
                <div>
                    <div style="display:flex; justify-content:space-between; font-size:10px; color: var(--text-low); margin-bottom:3px;">
                        <span>Cinematic (Dramatic)</span>
                        <span>${Math.round(cinematic * 100)}%</span>
                    </div>
                    <div style="height:3px; background: var(--border-glass); border-radius:2px; overflow:hidden;">
                        <div style="height:100%; width:${Math.round(cinematic * 100)}%; background:linear-gradient(90deg, var(--accent-rose), #fb7185); border-radius:2px;"></div>
                    </div>
                </div>
            </div>
        </div>
    `;

    grid.innerHTML = colAcousticHtml + colProductionHtml;
}

// Low Visual / Low GPU Performance Mode Toggle Controller
function initLowGpuMode() {
    const mainToggle = document.getElementById("settings-toggle-low-gpu");
    const mobileToggle = document.getElementById("mr-toggle-low-gpu");
    
    const isSavedLowGpu = localStorage.getItem("player-low-gpu-mode") === "true";
    document.body.classList.toggle("low-gpu-mode", isSavedLowGpu);
    
    if (mainToggle) mainToggle.checked = isSavedLowGpu;
    if (mobileToggle) mobileToggle.checked = isSavedLowGpu;

    function handleLowGpuChange(enabled) {
        document.body.classList.toggle("low-gpu-mode", enabled);
        localStorage.setItem("player-low-gpu-mode", enabled ? "true" : "false");
        if (mainToggle) mainToggle.checked = enabled;
        if (mobileToggle) mobileToggle.checked = enabled;
    }

    if (mainToggle) {
        mainToggle.addEventListener("change", (e) => handleLowGpuChange(e.target.checked));
    }
    if (mobileToggle) {
        mobileToggle.addEventListener("change", (e) => handleLowGpuChange(e.target.checked));
    }
}

// WASAPI Exclusive Mode Toggle Controller
function initWasapiExclusiveMode() {
    const mainToggle = document.getElementById("settings-toggle-wasapi-exclusive");
    const mobileToggle = document.getElementById("mr-toggle-wasapi-exclusive");

    const isExclusive = window.serverState?.preferences?.["dsp-wasapi_exclusive"] !== false;
    
    if (mainToggle) mainToggle.checked = isExclusive;
    if (mobileToggle) mobileToggle.checked = isExclusive;

    function handleWasapiExclusiveChange(enabled) {
        if (mainToggle) mainToggle.checked = enabled;
        if (mobileToggle) mobileToggle.checked = enabled;
        if (!state.pref) state.pref = {};
        state.pref["dsp-wasapi_exclusive"] = String(enabled);
        saveServerState("dsp-wasapi_exclusive", enabled);
        flushServerState();
        updateAudioQualityPillBanner();
    }

    if (mainToggle) {
        mainToggle.addEventListener("change", (e) => handleWasapiExclusiveChange(e.target.checked));
    }
    if (mobileToggle) {
        mobileToggle.addEventListener("change", (e) => handleWasapiExclusiveChange(e.target.checked));
    }
}

// Audio Output Hardware Device Dropdown Controller
// Audio Output Hardware Device Dropdown Controller
async function loadAudioDevices() {
    const mainSelect = document.getElementById("settings-select-audio-device");
    const mobileSelect = document.getElementById("mr-select-audio-device");
    if (!mainSelect && !mobileSelect) return;

    try {
        const res = await fetch("/api/audio/devices");
        if (!res.ok) return;
        const data = await res.json();
        const devices = data.devices || [];
        const selected = data.selected || "default";

        let optionsHtml = `<option value="default">Default System Audio Device</option>`;
        devices.forEach(d => {
            const label = `${d.name} (${d.hostapi}${d.default_samplerate ? ` - ${d.default_samplerate/1000} kHz` : ''})`;
            optionsHtml += `<option value="${d.id}" ${String(d.id) === String(selected) ? 'selected' : ''}>${label}</option>`;
        });

        if (mainSelect) {
            mainSelect.innerHTML = optionsHtml;
            mainSelect.value = selected;
            mainSelect.onchange = (e) => handleAudioDeviceChange(e.target.value);
        }
        if (mobileSelect) {
            mobileSelect.innerHTML = optionsHtml;
            mobileSelect.value = selected;
            mobileSelect.onchange = (e) => handleAudioDeviceChange(e.target.value);
        }

        const volTargetSelect = document.getElementById("settings-select-volume-target");
        if (volTargetSelect) {
            const wasapiName = data.exclusive_device || "Sonar Exclusive DAC";
            const curVal = data.volume_keys_target || "exclusive";
            volTargetSelect.innerHTML = `
                <option value="exclusive">🎵 Sonar Exclusive Music (${escapeHtml(wasapiName)})</option>
                <option value="null">🔇 Null Target Device (YouTube / System Audio)</option>
            `;
            volTargetSelect.value = curVal;
            if (!volTargetSelect.dataset.bound) {
                volTargetSelect.dataset.bound = "true";
                volTargetSelect.addEventListener("change", async (e) => {
                    const targetVal = e.target.value;
                    saveServerState("dsp-volume_keys_target", targetVal);
                    await flushServerState();
                    if (typeof showNotification === "function") {
                        showNotification(targetVal === "null" ? "🔇 System Volume Keys now control Null Target Device (YouTube / External Audio)" : `🎵 System Volume Keys now control Sonar Exclusive Music Output (${wasapiName})`);
                    }
                });
            }
        }

        const dynRedirectToggle = document.getElementById("settings-toggle-dynamic-redirect");
        if (dynRedirectToggle) {
            const isDynEnabled = data.dynamic_redirect_enabled !== undefined ? !!data.dynamic_redirect_enabled : true;
            dynRedirectToggle.checked = isDynEnabled;
            if (!dynRedirectToggle.dataset.bound) {
                dynRedirectToggle.dataset.bound = "true";
                dynRedirectToggle.addEventListener("change", async (e) => {
                    const isDynOn = e.target.checked;
                    saveServerState("dsp-dynamic_redirect_enabled", isDynOn);
                    await flushServerState();
                    if (typeof showNotification === "function") {
                        showNotification(isDynOn ? "🔀 Dynamic Redirection ON — System audio redirected only when YouTube/external sound is active" : "🔊 Dynamic Redirection OFF");
                    }
                });
            }
        }

        const volStepSelect = document.getElementById("settings-select-volume-step");
        if (volStepSelect) {
            const curStep = data.volume_step_size || 2;
            volStepSelect.value = String(curStep);
            state.volumeStepSize = curStep;
            if (!volStepSelect.dataset.bound) {
                volStepSelect.dataset.bound = "true";
                volStepSelect.addEventListener("change", async (e) => {
                    const stepVal = Number(e.target.value) || 2;
                    state.volumeStepSize = stepVal;
                    saveServerState("dsp-volume_step_size", stepVal);
                    await flushServerState();
                    if (typeof showNotification === "function") {
                        showNotification(`⚡ Volume Step Size set to ${stepVal}%`);
                    }
                });
            }
        }

        const perPageSelect = document.getElementById("settings-select-per-page");
        const tablePerPageSelect = document.getElementById("per-page-select");
        if (perPageSelect) {
            const curPerPage = data.per_page || 50;
            perPageSelect.value = String(curPerPage);
            if (tablePerPageSelect) tablePerPageSelect.value = String(curPerPage);
            if (!perPageSelect.dataset.bound) {
                perPageSelect.dataset.bound = "true";
                perPageSelect.addEventListener("change", async (e) => {
                    const perPageVal = Number(e.target.value) || 50;
                    if (tablePerPageSelect) tablePerPageSelect.value = String(perPageVal);
                    saveServerState("library-per_page", perPageVal);
                    await flushServerState();
                    if (typeof loadTracks === "function") loadTracks(1);
                    if (typeof showNotification === "function") {
                        showNotification(`📄 Default Page Size set to ${perPageVal} / page`);
                    }
                });
            }
        }

        // Always populate the Null Target Device dropdown when settings load
        await loadVoidDevices();

    } catch (e) {
        console.error("Error loading audio output devices:", e);
    }
}

async function handleAudioDeviceChange(value) {
    const mainSelect = document.getElementById("settings-select-audio-device");
    const mobileSelect = document.getElementById("mr-select-audio-device");
    if (mainSelect) mainSelect.value = value;
    if (mobileSelect) mobileSelect.value = value;
    saveServerState("dsp-audio_device", value);
    await flushServerState();
    // Actively refresh the void devices dropdown & description to reflect the new target device
    await loadVoidDevices();
}

// Null Target Device Dropdown Controller
async function loadVoidDevices() {
    const sel = document.getElementById("settings-select-void-device");
    if (!sel) return;
    try {
        const res = await fetch("/api/audio/void-devices");
        if (!res.ok) return;
        const data = await res.json();
        const devices = data.devices || [];
        const selected = data.selected || "auto";
        const wasapiDevName = data.exclusive_device || "";

        // Update description element with the current WASAPI output device name for context
        const descEl = document.getElementById("void-device-desc") || sel.closest(".settings-row")?.querySelector(".desc");
        if (descEl && wasapiDevName) {
            descEl.innerHTML = `Select the silent endpoint that receives audio when <strong>Null Mode is ON</strong>. 
                Your WASAPI Output Device (<strong style="color:var(--accent-cyan)">${escapeHtml(wasapiDevName)}</strong>) 
                is excluded — pick a different, ideally <em>NotPresent/Unplugged</em> endpoint for true silence. 
                Last selection is remembered across restarts.`;
        }

        // Build options grouped by Windows endpoint state
        const stateOrder = ["Active", "NotPresent", "Unplugged", "Disabled"];
        const stateLabel = {
            "Active":     "🔊 Active Endpoints (will produce sound)",
            "NotPresent": "🔇 Not Connected — True Silent Void",
            "Unplugged":  "🔌 Unplugged — Silent Void",
            "Disabled":   "⛔ Disabled Endpoints"
        };

        let html = `<option value="auto">🔇 Auto (Digital Output / NVIDIA preferred)</option>`;
        for (const state of stateOrder) {
            const group = devices.filter(d => d.state === state);
            if (!group.length) continue;
            html += `<optgroup label="${stateLabel[state] || state}">`;
            group.forEach(d => {
                const isSel = d.id === selected ? "selected" : "";
                // Mark ideal void candidates
                const isIdeal = (d.state === "NotPresent" || d.state === "Unplugged") && d.is_void_candidate;
                const badge = isIdeal ? " ✅ Ideal Null Target" : "";
                html += `<option value="${escapeHtml(d.id)}" ${isSel}>${escapeHtml(d.name)}${badge}</option>`;
            });
            html += `</optgroup>`;
        }

        sel.innerHTML = html;
        // Restore last saved selection
        if (selected && selected !== "auto") {
            sel.value = selected;
        }
        sel.onchange = async (e) => {
            saveServerState("dsp-void_device_id", e.target.value);
            await flushServerState();
            if (typeof showNotification === "function") {
                const opt = sel.options[sel.selectedIndex];
                showNotification(`🔇 Null Target set to: ${opt?.text || e.target.value}`);
            }
        };
    } catch (e) {
        console.error("Error loading null target devices:", e);
    }
}


// 10-Band Graphic Equalizer Controller
// Initialized at top level: EQ_FREQUENCIES, EQ_PRESETS

// Initialized at top level: currentEq10Band, customEqPresets

function loadCustomEqPresets() {
    const saved = window.serverState?.preferences?.["dsp-custom_eq_presets"];
    if (saved) {
        if (typeof saved === "object" && saved !== null) {
            customEqPresets = saved;
        } else if (typeof saved === "string") {
            try { customEqPresets = JSON.parse(saved); } catch (e) { customEqPresets = {}; }
        }
    }
    const defaultKey = window.serverState?.preferences?.["dsp-default-10band-preset-key"];
    renderEqPresetOptions(defaultKey || null);

    if (defaultKey) {
        applyEqPreset(defaultKey);
    } else {
        const savedEq = window.serverState?.preferences?.["dsp-eq_10band"];
        if (savedEq) {
            let parsed = null;
            if (Array.isArray(savedEq) && savedEq.length === 10) parsed = savedEq.map(Number);
            else if (typeof savedEq === "string") {
                try {
                    const arr = JSON.parse(savedEq);
                    if (Array.isArray(arr) && arr.length === 10) parsed = arr.map(Number);
                } catch (e) {}
            }
            if (parsed) {
                currentEq10Band = [...parsed];
                EQ_FREQUENCIES.forEach((_, idx) => {
                    const val = currentEq10Band[idx] || 0;
                    syncDesktopEqSlider(idx, val);
                    syncMobileEqSlider(idx, val);
                });
            }
        }
    }
}

function renderEqPresetOptions(activeKey = null) {
    const desktopSelect = document.getElementById("dsp-eq-preset-select");
    const mobileSelect = document.getElementById("mr-eq-preset-select");
    if (!desktopSelect && !mobileSelect) return;

    let builtinHtml = `
        <option value="flat">Flat (Default)</option>
        <option value="bass">Bass Boost</option>
        <option value="vocal">Vocal Clarity</option>
        <option value="treble">Treble Boost</option>
        <option value="electronic">Electronic / EDM</option>
        <option value="rock">Rock</option>
        <option value="pop">Pop</option>
        <option value="acoustic">Acoustic</option>
        <option value="classical">Classical</option>
    `;

    let customHtml = "";
    Object.keys(customEqPresets).forEach(name => {
        customHtml += `<option value="custom:${name}">⭐ ${name}</option>`;
    });

    const fullHtml = builtinHtml + (customHtml ? `<optgroup label="Custom Saved Presets">${customHtml}</optgroup>` : '');

    if (desktopSelect) {
        desktopSelect.innerHTML = fullHtml;
        if (activeKey) desktopSelect.value = activeKey;
    }
    if (mobileSelect) {
        mobileSelect.innerHTML = fullHtml;
        if (activeKey) mobileSelect.value = activeKey;
    }

    updateDeleteButtonState(activeKey || (desktopSelect ? desktopSelect.value : "flat"));
}

function updateDeleteButtonState(presetKey) {
    const desktopDel = document.getElementById("dsp-eq-btn-delete");
    const mobileDel = document.getElementById("mr-eq-btn-delete");
    const isCustom = presetKey && presetKey.startsWith("custom:");

    if (desktopDel) desktopDel.style.display = isCustom ? "inline-flex" : "none";
    if (mobileDel) mobileDel.style.display = isCustom ? "inline-block" : "none";
}

function saveCustomEqPreset(name) {
    if (!name || !name.trim()) {
        alert("Please enter a name for your custom EQ preset.");
        return;
    }
    const cleanName = name.trim();
    customEqPresets[cleanName] = [...currentEq10Band];
    saveServerState("dsp-custom_eq_presets", JSON.stringify(customEqPresets));
    flushServerState();
    renderEqPresetOptions(`custom:${cleanName}`);

    const desktopInput = document.getElementById("dsp-eq-save-name");
    const mobileInput = document.getElementById("mr-eq-save-name");
    if (desktopInput) desktopInput.value = "";
    if (mobileInput) mobileInput.value = "";
    alert(`EQ Preset "${cleanName}" saved successfully!`);
}

function deleteCustomEqPreset(presetKey) {
    if (!presetKey || !presetKey.startsWith("custom:")) return;
    const name = presetKey.replace("custom:", "");
    delete customEqPresets[name];
    saveServerState("dsp-custom_eq_presets", JSON.stringify(customEqPresets));
    flushServerState();
    renderEqPresetOptions("flat");
    applyEqPreset("flat");
}

function init10BandEqualizer() {
    const desktopContainer = document.getElementById("dsp-10band-container");
    const mobileContainer = document.getElementById("mr-10band-container");
    const desktopPreset = document.getElementById("dsp-eq-preset-select");
    const mobilePreset = document.getElementById("mr-eq-preset-select");

    const savedEq = window.serverState?.preferences?.["dsp-eq_10band"];
    if (savedEq) {
        if (Array.isArray(savedEq) && savedEq.length === 10) {
            currentEq10Band = savedEq.map(Number);
        } else if (typeof savedEq === "string") {
            try {
                const parsed = JSON.parse(savedEq);
                if (Array.isArray(parsed) && parsed.length === 10) {
                    currentEq10Band = parsed.map(Number);
                }
            } catch (e) {}
        }
    }

    // Render Desktop 10-Band Vertical Sliders
    if (desktopContainer) {
        let html = "";
        EQ_FREQUENCIES.forEach((freq, idx) => {
            const val = currentEq10Band[idx] || 0;
            html += `
                <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                    <span id="dsp-eq-val-${idx}" style="font-size: 10px; font-weight: 700; color: var(--accent-cyan); font-family: monospace;">${val > 0 ? '+' : ''}${val} dB</span>
                    <input type="range" id="dsp-eq-slider-${idx}" min="-30" max="30" step="1" value="${val}"
                           orient="vertical" class="eq-vert-slider"
                           data-band="${idx}">
                    <span style="font-size: 10px; color: var(--text-mid); font-weight: 600;">${freq}</span>
                </div>
            `;
        });
        desktopContainer.innerHTML = html;

        EQ_FREQUENCIES.forEach((_, idx) => {
            const slider = document.getElementById(`dsp-eq-slider-${idx}`);
            if (slider) {
                slider.addEventListener("input", (e) => {
                    const bandVal = parseFloat(e.target.value);
                    currentEq10Band[idx] = bandVal;
                    const valSpan = document.getElementById(`dsp-eq-val-${idx}`);
                    if (valSpan) valSpan.textContent = `${bandVal > 0 ? '+' : ''}${bandVal} dB`;
                    syncMobileEqSlider(idx, bandVal);
                    sendEqState();
                });
            }
        });
    }

    // Render Mobile Remote 10-Band Horizontal Sliders
    if (mobileContainer) {
        let mobileHtml = "";
        EQ_FREQUENCIES.forEach((freq, idx) => {
            const val = currentEq10Band[idx] || 0;
            mobileHtml += `
                <div style="display: flex; align-items: center; gap: 8px;">
                    <span style="font-size: 11px; color: var(--mr-mid); width: 42px; font-weight: 600;">${freq}</span>
                    <input type="range" class="mr-dsp-slider" id="mr-eq-slider-${idx}" min="-30" max="30" step="1" value="${val}" style="flex: 1;" data-band="${idx}">
                    <span id="mr-eq-val-${idx}" style="font-size: 11px; font-weight: 700; color: var(--mr-cyan); width: 40px; text-align: right; font-family: monospace;">${val > 0 ? '+' : ''}${val} dB</span>
                </div>
            `;
        });
        mobileContainer.innerHTML = mobileHtml;

        EQ_FREQUENCIES.forEach((_, idx) => {
            const slider = document.getElementById(`mr-eq-slider-${idx}`);
            if (slider) {
                slider.addEventListener("input", (e) => {
                    const bandVal = parseFloat(e.target.value);
                    currentEq10Band[idx] = bandVal;
                    const valSpan = document.getElementById(`mr-eq-val-${idx}`);
                    if (valSpan) valSpan.textContent = `${bandVal > 0 ? '+' : ''}${bandVal} dB`;
                    syncDesktopEqSlider(idx, bandVal);
                    sendEqState();
                });
            }
        });
    }

    // Load Custom Presets
    loadCustomEqPresets();

    // Handle Presets selection
    if (desktopPreset) {
        desktopPreset.addEventListener("change", (e) => applyEqPreset(e.target.value));
    }
    if (mobilePreset) {
        mobilePreset.addEventListener("change", (e) => applyEqPreset(e.target.value));
    }

    // Handle Save Presets
    const dspSaveBtn = document.getElementById("dsp-eq-btn-save");
    const dspSaveInput = document.getElementById("dsp-eq-save-name");
    const mrSaveBtn = document.getElementById("mr-eq-btn-save");
    const mrSaveInput = document.getElementById("mr-eq-save-name");

    if (dspSaveBtn && dspSaveInput) {
        dspSaveBtn.addEventListener("click", () => saveCustomEqPreset(dspSaveInput.value));
        dspSaveInput.addEventListener("keydown", (e) => {
            if (e.key === "Enter") saveCustomEqPreset(dspSaveInput.value);
        });
    }
    if (mrSaveBtn && mrSaveInput) {
        mrSaveBtn.addEventListener("click", () => saveCustomEqPreset(mrSaveInput.value));
        mrSaveInput.addEventListener("keydown", (e) => {
            if (e.key === "Enter") saveCustomEqPreset(mrSaveInput.value);
        });
    }

    // Handle Set as Default 10-Band EQ Preset
    const dspSetDefaultBtn = document.getElementById("dsp-eq-btn-set-default");
    if (dspSetDefaultBtn) {
        dspSetDefaultBtn.addEventListener("click", () => {
            const currentVal = desktopPreset ? desktopPreset.value : "flat";
            saveServerState("dsp-default-10band-preset-key", currentVal);
            saveServerState("dsp-eq_10band", currentEq10Band);
            flushServerState();
            const text = desktopPreset && desktopPreset.options[desktopPreset.selectedIndex] ? desktopPreset.options[desktopPreset.selectedIndex].text : currentVal;
            alert(`10-Band EQ Preset "${text}" set as your default on startup!`);
        });
    }

    // Handle Delete Preset
    const dspDelBtn = document.getElementById("dsp-eq-btn-delete");
    const mrDelBtn = document.getElementById("mr-eq-btn-delete");

    if (dspDelBtn) {
        dspDelBtn.addEventListener("click", () => {
            const currentVal = desktopPreset ? desktopPreset.value : "";
            deleteCustomEqPreset(currentVal);
        });
    }
    if (mrDelBtn) {
        mrDelBtn.addEventListener("click", () => {
            const currentVal = mobilePreset ? mobilePreset.value : "";
            deleteCustomEqPreset(currentVal);
        });
    }
}

function syncDesktopEqSlider(idx, val) {
    const slider = document.getElementById(`dsp-eq-slider-${idx}`);
    const valSpan = document.getElementById(`dsp-eq-val-${idx}`);
    if (slider) slider.value = val;
    if (valSpan) valSpan.textContent = `${val > 0 ? '+' : ''}${val}dB`;
}

function syncMobileEqSlider(idx, val) {
    const slider = document.getElementById(`mr-eq-slider-${idx}`);
    const valSpan = document.getElementById(`mr-eq-val-${idx}`);
    if (slider) slider.value = val;
    if (valSpan) valSpan.textContent = `${val > 0 ? '+' : ''}${val}dB`;
}

function applyEqPreset(presetKey) {
    let preset = null;
    if (presetKey && presetKey.startsWith("custom:")) {
        const name = presetKey.replace("custom:", "");
        preset = customEqPresets[name];
    } else {
        preset = EQ_PRESETS[presetKey];
    }
    if (!preset || !Array.isArray(preset)) return;

    currentEq10Band = [...preset];

    EQ_FREQUENCIES.forEach((_, idx) => {
        const val = currentEq10Band[idx] || 0;
        syncDesktopEqSlider(idx, val);
        syncMobileEqSlider(idx, val);
    });

    const desktopPreset = document.getElementById("dsp-eq-preset-select");
    const mobilePreset = document.getElementById("mr-eq-preset-select");
    if (desktopPreset) desktopPreset.value = presetKey;
    if (mobilePreset) mobilePreset.value = presetKey;

    updateDeleteButtonState(presetKey);
    sendEqState();
}

// Initialized at top level: eqSendTimeout
function sendEqState() {
    saveServerState("dsp-eq_10band", currentEq10Band);
    if (eqSendTimeout) clearTimeout(eqSendTimeout);
    eqSendTimeout = setTimeout(() => {
        fetch("/api/player/dsp", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ eq_10band: currentEq10Band })
        }).catch(e => console.error("Error pushing 10-band EQ:", e));
    }, 100);
}

// Master System Volume Controller
function initSystemVolumeController() {
    const slider = document.getElementById("system-volume-slider");
    const valSpan = document.getElementById("system-volume-val");
    const miniSlider = document.getElementById("mini-system-volume-slider");
    if (!slider) return;

    let isDragging = false;

    function updateSysProgress(vol) {
        const mainSlider = document.getElementById("system-volume-slider");
        const miniSliderEl = document.getElementById("mini-system-volume-slider");
        const overlaySliderEl = document.getElementById("overlay-system-volume-slider");
        const mainLabel = document.getElementById("system-volume-val");
        const overlayLabel = document.getElementById("overlay-system-volume-label");

        if (mainSlider) {
            mainSlider.value = vol;
            mainSlider.style.setProperty("--sys-progress", `${vol}%`);
        }
        if (miniSliderEl) {
            miniSliderEl.value = vol;
            miniSliderEl.style.setProperty("--sys-progress", `${vol}%`);
        }
        if (overlaySliderEl) {
            overlaySliderEl.value = vol;
            overlaySliderEl.style.setProperty("--sys-progress", `${vol}%`);
        }
        if (mainLabel) mainLabel.textContent = `${vol}%`;
        if (overlayLabel) overlayLabel.textContent = `${vol}%`;
    }

    async function fetchSystemVolume() {
        if (isDragging) return; // Don't overwrite slider while user is dragging
        try {
            const res = await fetch("/api/system/volume");
            if (res.ok) {
                const data = await res.json();
                if (data.success && typeof data.system_volume === "number") {
                    const vol = data.system_volume;
                    if (!isDragging && document.activeElement !== slider) {
                        slider.value = vol;
                        updateSysProgress(vol);
                    }
                    if (miniSlider && document.activeElement !== miniSlider) miniSlider.value = vol;
                    if (valSpan) valSpan.textContent = `${vol}%`;
                }
            }
        } catch (e) {
            console.error("Error fetching system volume:", e);
        }
    }

    function sendVolumeToServer(vol) {
        const value = Math.max(0, Math.min(100, Math.round(Number(vol) || 0)));
        if (isWsConnected && ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: "command", command: `system_volume:${value}` }));
            return;
        }
        fetch("/api/system/volume", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ volume: value })
        }).catch(err => console.error("Error setting system volume:", err));
    }

    let pendingVolume = null;
    let volumeSendTimer = null;
    function scheduleSystemVolume(vol, flush = false) {
        pendingVolume = vol;
        if (flush) {
            if (volumeSendTimer) clearTimeout(volumeSendTimer);
            volumeSendTimer = null;
            const value = pendingVolume;
            pendingVolume = null;
            sendVolumeToServer(value);
            return;
        }
        if (volumeSendTimer) return;
        volumeSendTimer = setTimeout(() => {
            volumeSendTimer = null;
            if (pendingVolume !== null) {
                const value = pendingVolume;
                pendingVolume = null;
                sendVolumeToServer(value);
            }
        }, 35);
    }

    slider.addEventListener("input", (e) => {
        const vol = parseInt(e.target.value) || 0;
        updateSysProgress(vol);
        if (valSpan) valSpan.textContent = `${vol}%`;
        if (miniSlider) {
            miniSlider.value = vol;
            miniSlider.style.setProperty("--sys-progress", `${vol}%`);
        }
        isDragging = true;
        scheduleSystemVolume(vol);
    });

    slider.addEventListener("change", (e) => {
        // Final value on release — send immediately
        const vol = parseInt(e.target.value) || 0;
        updateSysProgress(vol);
        scheduleSystemVolume(vol, true);
        setTimeout(() => { isDragging = false; }, 300);
    });

    // Also handle mouseup/touchend to clear dragging state
    slider.addEventListener("mouseup", () => { setTimeout(() => { isDragging = false; }, 300); });
    slider.addEventListener("touchend", () => { setTimeout(() => { isDragging = false; }, 300); });

    if (miniSlider) {
        miniSlider.addEventListener("input", (e) => {
            const vol = parseInt(e.target.value) || 0;
            slider.value = vol;
            updateSysProgress(vol);
            if (valSpan) valSpan.textContent = `${vol}%`;
            isDragging = true;
            scheduleSystemVolume(vol);
        });
        miniSlider.addEventListener("change", (e) => {
            const vol = parseInt(e.target.value) || 0;
            updateSysProgress(vol);
            scheduleSystemVolume(vol, true);
            setTimeout(() => { isDragging = false; }, 300);
        });
    }

    fetchSystemVolume();
    setInterval(fetchSystemVolume, 4000);
}

// Sample Rate Unsupported Modal Controller
function initSampleRateModal() {
    const modal = document.getElementById("sample-rate-modal");
    const msgEl = document.getElementById("sample-rate-modal-msg");
    const btnDefault = document.getElementById("sr-modal-btn-default");
    const btnShared = document.getElementById("sr-modal-btn-shared");
    const btnClose = document.getElementById("sr-modal-btn-close");

    if (!modal) return;

    if (btnDefault) {
        btnDefault.onclick = async () => {
            modal.style.display = "none";
            saveServerState("dsp-audio_device", "default");
            await flushServerState();
            await loadAudioDevices();
            await loadVoidDevices();
            fetch("/api/player/resume").catch(() => {});
        };
    }

    if (btnShared) {
        btnShared.onclick = async () => {
            modal.style.display = "none";
            saveServerState("dsp-wasapi_exclusive", false);
            await flushServerState();
            const mainToggle = document.getElementById("settings-toggle-wasapi-exclusive");
            const mobileToggle = document.getElementById("mr-toggle-wasapi-exclusive");
            if (mainToggle) mainToggle.checked = false;
            if (mobileToggle) mobileToggle.checked = false;
            fetch("/api/player/resume").catch(() => {});
        };
    }

    if (btnClose) {
        btnClose.onclick = () => {
            modal.style.display = "none";
        };
    }

    let lastShownErrorTime = 0;
    async function checkSampleRateError() {
        try {
            const res = await fetch("/api/player/status");
            if (!res.ok) return;
            const status = await res.json();
            if (status.last_error && status.last_error.type === "sample_rate_unsupported") {
                const err = status.last_error;
                // Only show if not shown recently
                const now = Date.now();
                if (now - lastShownErrorTime > 5000 && modal.style.display !== "flex") {
                    lastShownErrorTime = now;
                    msgEl.innerHTML = `The selected device <strong>"${err.device_name}"</strong> rejected <strong>${err.sample_rate} Hz</strong> playback.<br><br>
                        💡 <strong>Note:</strong> If your boAt / USB DAC supports 24-bit 44.1 kHz, open <strong>Windows Sound Control Panel</strong> (Device Properties ➔ Advanced ➔ Default Format) and select <strong>24-bit, 44100 Hz</strong>, or choose a resolution below:`;
                    modal.style.display = "flex";
                }
            }
        } catch (e) {}
    }

    setInterval(checkSampleRateError, 2500);
}

if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => {
        initLowGpuMode();
        initWasapiExclusiveMode();
        loadAudioDevices();
        loadVoidDevices();
        init10BandEqualizer();
        initSystemVolumeController();
        initSampleRateModal();
        setupNavigation();
        initRemoteControlUI();
    });
} else {
    initLowGpuMode();
    initWasapiExclusiveMode();
    loadAudioDevices();
    loadVoidDevices();
    init10BandEqualizer();
    initSystemVolumeController();
    initSampleRateModal();
    setupNavigation();
    initRemoteControlUI();
}




// MusicBee Seamless Navigation History Engine
var navHistoryStack = [];
var navHistoryIndex = -1;
var currentGroupingState = { groupType: null, groupText: null };

function pushNavHistoryState(stateObj) {
    if (!stateObj) return;

    // Save current scroll position on existing top item before pushing new state
    if (navHistoryIndex >= 0 && navHistoryStack[navHistoryIndex]) {
        const current = navHistoryStack[navHistoryIndex];

        // Deduplicate identical state pushes
        if (current.type === stateObj.type &&
            current.workspaceId === stateObj.workspaceId &&
            current.subtabId === stateObj.subtabId &&
            current.groupType === stateObj.groupType &&
            current.value === stateObj.value &&
            current.query === stateObj.query) {
            return;
        }

        const activeWs = document.querySelector(".workspace-panel.active") || document.querySelector(".table-container");
        if (activeWs) {
            current.scrollTop = activeWs.scrollTop || 0;
        }
    }

    navHistoryStack.splice(navHistoryIndex + 1);
    navHistoryStack.push(stateObj);
    navHistoryIndex = navHistoryStack.length - 1;
    updateNavHistoryButtonsUI();
}

function updateNavHistoryButtonsUI() {
    const backBtns = document.querySelectorAll("#btn-history-back, .btn-history-back, .btn-nav-back");
    const fwdBtns = document.querySelectorAll("#btn-history-forward, .btn-history-forward, .btn-nav-fwd");

    const canGoBack = navHistoryIndex > 0;
    const canGoFwd = navHistoryIndex < navHistoryStack.length - 1;

    backBtns.forEach(btn => {
        btn.disabled = !canGoBack;
        btn.style.opacity = canGoBack ? "1" : "0.35";
        btn.style.cursor = canGoBack ? "pointer" : "default";
    });

    fwdBtns.forEach(btn => {
        btn.disabled = !canGoFwd;
        btn.style.opacity = canGoFwd ? "1" : "0.35";
        btn.style.cursor = canGoFwd ? "pointer" : "default";
    });
}

function goNavBack() {
    if (navHistoryIndex > 0) {
        navHistoryIndex--;
        applyNavHistoryState(navHistoryStack[navHistoryIndex]);
        updateNavHistoryButtonsUI();
    }
}

function goNavForward() {
    if (navHistoryIndex < navHistoryStack.length - 1) {
        navHistoryIndex++;
        applyNavHistoryState(navHistoryStack[navHistoryIndex]);
        updateNavHistoryButtonsUI();
    }
}

function applyNavHistoryState(stateObj) {
    if (!stateObj) return;

    if (stateObj.type === "workspace") {
        switchWorkspace(stateObj.workspaceId, stateObj.subtabId, false);
    } else if (stateObj.type === "group_explorer") {
        switchWorkspace("workspace-library", null, false);
        showGroupExplorerMode(stateObj.groupType, stateObj.text, false);
    } else if (stateObj.type === "group_detail") {
        switchWorkspace("workspace-library", null, false);
        showGroupDetailMode(stateObj.column, stateObj.value, stateObj.groupType, stateObj.text, false);
    } else if (stateObj.type === "normal_list" || stateObj.type === "library_search") {
        switchWorkspace("workspace-library", null, false);
        const searchInput = document.getElementById("search-input");
        const queryVal = stateObj.query || "";
        if (searchInput) searchInput.value = queryVal;
        state.searchQuery = queryVal;
        state.currentPage = stateObj.page || 1;
        state.activeGroupingField = stateObj.groupingField || null;
        state.activeGroupingValue = stateObj.groupingValue || null;
        if (typeof updateSearchClearBtnVisibility === "function") updateSearchClearBtnVisibility();
        if (typeof updateAdvancedFiltersBadge === "function") updateAdvancedFiltersBadge();
        showNormalListMode(false);
    }

    // Restore saved scroll position seamlessly
    if (typeof stateObj.scrollTop === "number") {
        setTimeout(() => {
            const activeWs = document.querySelector(".workspace-panel.active") || document.querySelector(".table-container");
            if (activeWs) activeWs.scrollTop = stateObj.scrollTop;
        }, 30);
    }
}

// Global click delegate for all back/forward buttons
document.addEventListener("click", (e) => {
    const backBtn = e.target.closest("#btn-history-back, .btn-history-back, .btn-nav-back");
    if (backBtn) {
        e.preventDefault();
        goNavBack();
        return;
    }

    const fwdBtn = e.target.closest("#btn-history-forward, .btn-history-forward, .btn-nav-fwd");
    if (fwdBtn) {
        e.preventDefault();
        goNavForward();
        return;
    }
});

// Hardware Mouse Back (Button 3) & Forward (Button 4) Support
document.addEventListener("mouseup", (e) => {
    if (e.button === 3) {
        e.preventDefault();
        goNavBack();
    } else if (e.button === 4) {
        e.preventDefault();
        goNavForward();
    }
});

function showNormalListMode(shouldPushState = true) {
    const groupingBar = document.getElementById("grouping-header-bar");
    const tableContainer = document.querySelector(".table-container");
    const paginationPanel = document.querySelector(".panel-footer");
    const detailBreadcrumb = document.getElementById("group-detail-breadcrumb");
    const groupingLabel = document.getElementById("current-grouping-label");

    if (groupingBar) groupingBar.style.display = "none";
    if (tableContainer) tableContainer.style.display = "block";
    if (paginationPanel) paginationPanel.style.display = "flex";
    if (detailBreadcrumb) detailBreadcrumb.style.display = "none";
    if (groupingLabel) groupingLabel.textContent = "None";

    delete state.activeGroupingField;
    delete state.activeGroupingValue;
    currentGroupingState = { groupType: null, groupText: null };

    if (shouldPushState) {
        pushNavHistoryState({ type: "normal_list" });
    }
    loadTracks();
}

async function showGroupExplorerMode(groupType, text, shouldPushState = true) {
    const groupingBar = document.getElementById("grouping-header-bar");
    const tableContainer = document.querySelector(".table-container");
    const paginationPanel = document.querySelector(".panel-footer");
    const detailBreadcrumb = document.getElementById("group-detail-breadcrumb");
    const groupingLabel = document.getElementById("current-grouping-label");
    const groupingTitle = document.getElementById("grouping-header-title");
    const groupingGrid = document.getElementById("grouping-chips-container");
    const groupingBadge = document.getElementById("grouping-count-badge");

    if (groupType === "none") {
        showNormalListMode(shouldPushState);
        return;
    }

    currentGroupingState = { groupType, groupText: text };

    if (groupingLabel) groupingLabel.textContent = text;
    if (groupingTitle) groupingTitle.textContent = `${text} Explorer`;
    if (groupingBar) groupingBar.style.display = "block";
    if (tableContainer) tableContainer.style.display = "none";
    if (paginationPanel) paginationPanel.style.display = "none";
    if (detailBreadcrumb) detailBreadcrumb.style.display = "none";

    if (groupingGrid) groupingGrid.innerHTML = `<div style="grid-column: 1/-1; color: var(--text-muted); font-size: 14px; padding: 40px; text-align: center;"><i class="fa-solid fa-spinner fa-spin"></i> Loading 100% Full-Screen ${text} Explorer...</div>`;

    if (shouldPushState) {
        pushNavHistoryState({ type: "group_explorer", groupType, text });
    }

    try {
        const res = await fetch(`/api/grouping?by=${groupType}`);
        if (res.ok) {
            const data = await res.json();
            const groups = data.groups || [];

            if (groupingBadge) groupingBadge.textContent = `${groups.length} ${text.toLowerCase()}${groups.length !== 1 ? 's' : ''}`;

            if (groups.length === 0) {
                groupingGrid.innerHTML = `<div style="grid-column: 1/-1; color: var(--text-muted); font-size: 14px; padding: 40px; text-align: center;">No ${text} items found</div>`;
                return;
            }

            groupingGrid.innerHTML = "";
            groups.forEach(g => {
                const nameStr = g.name || "Unknown";
                const artUrl = g.trackId ? `/api/art?id=${g.trackId}` : "";
                
                const card = document.createElement("div");
                card.className = "grid-card grouping-card";
                card.innerHTML = `
                    <img src="${artUrl}" class="grid-card-img" onerror="handleArtError(this)">
                    <div class="grid-card-title" title="${escapeHtml(nameStr)}">${escapeHtml(nameStr)}</div>
                    <div class="grid-card-artist">${escapeHtml(text)} · ${g.count} tracks</div>
                `;

                card.addEventListener("click", () => {
                    showGroupDetailMode(data.column, nameStr, groupType, text, true);
                });
                groupingGrid.appendChild(card);
            });
        }
    } catch (err) {
        console.error("Error fetching grouping data:", err);
        if (groupingGrid) groupingGrid.innerHTML = `<div style="grid-column: 1/-1; color: #f87171; font-size: 14px; padding: 40px; text-align: center;">Failed to load ${text} Explorer</div>`;
    }
}

function showGroupDetailMode(col, val, groupType, text, shouldPushState = true) {
    const groupingBar = document.getElementById("grouping-header-bar");
    const tableContainer = document.querySelector(".table-container");
    const paginationPanel = document.querySelector(".panel-footer");
    const detailBreadcrumb = document.getElementById("group-detail-breadcrumb");
    const breadcrumbTitle = document.getElementById("breadcrumb-title");
    const breadcrumbBackText = document.getElementById("breadcrumb-back-text");

    if (groupingBar) groupingBar.style.display = "none";
    if (tableContainer) tableContainer.style.display = "block";
    if (paginationPanel) paginationPanel.style.display = "flex";
    if (detailBreadcrumb) detailBreadcrumb.style.display = "flex";
    if (breadcrumbTitle) breadcrumbTitle.textContent = val;
    if (breadcrumbBackText) breadcrumbBackText.textContent = `Back to ${text} Explorer`;

    state.activeGroupingField = col;
    state.activeGroupingValue = val;
    state.currentPage = 1;

    if (shouldPushState) {
        pushNavHistoryState({ type: "group_detail", column: col, value: val, groupType, text });
    }
    loadTracks();
}

function setupGroupingMenu() {
    const btnGroupingMenu = document.getElementById("btn-grouping-menu");
    const popup = document.getElementById("grouping-menu-popup");
    const btnCloseGroupingBar = document.getElementById("btn-close-grouping-bar");
    const btnBreadcrumbBack = document.getElementById("btn-breadcrumb-back");
    const btnCloseGroupDetail = document.getElementById("btn-close-group-detail");

    // Initialize history stack with normal list mode if empty
    if (navHistoryStack.length === 0) {
        pushNavHistoryState({ type: "normal_list" });
    }
    updateNavHistoryButtonsUI();

    // Global click delegate for all back/forward buttons
    document.addEventListener("click", (e) => {
        const backBtn = e.target.closest("#btn-history-back, .btn-history-back, .btn-nav-back");
        if (backBtn) {
            e.preventDefault();
            if (navHistoryIndex > 0) {
                navHistoryIndex--;
                applyNavHistoryState(navHistoryStack[navHistoryIndex]);
                updateNavHistoryButtonsUI();
            }
            return;
        }

        const fwdBtn = e.target.closest("#btn-history-forward, .btn-history-forward, .btn-nav-fwd");
        if (fwdBtn) {
            e.preventDefault();
            if (navHistoryIndex < navHistoryStack.length - 1) {
                navHistoryIndex++;
                applyNavHistoryState(navHistoryStack[navHistoryIndex]);
                updateNavHistoryButtonsUI();
            }
            return;
        }
    });

    // Keyboard shortcut: Alt+Left = Back, Alt+Right = Forward, and Hardware Volume Keys
    document.addEventListener("keydown", (e) => {
        if (e.key === "AudioVolumeUp" || e.code === "VolumeUp") {
            const volSlider = document.getElementById("audio-volume-slider");
            if (volSlider) {
                const cur = Number(volSlider.value) || 80;
                const next = Math.min(100, cur + 5);
                volSlider.value = next;
                volSlider.dispatchEvent(new Event("input", { bubbles: true }));
                volSlider.dispatchEvent(new Event("change", { bubbles: true }));
            }
        } else if (e.key === "AudioVolumeDown" || e.code === "VolumeDown") {
            const volSlider = document.getElementById("audio-volume-slider");
            if (volSlider) {
                const cur = Number(volSlider.value) || 80;
                const next = Math.max(0, cur - 5);
                volSlider.value = next;
                volSlider.dispatchEvent(new Event("input", { bubbles: true }));
                volSlider.dispatchEvent(new Event("change", { bubbles: true }));
            }
        } else if (e.key === "AudioVolumeMute" || e.code === "VolumeMute") {
            const volSlider = document.getElementById("audio-volume-slider");
            if (volSlider) {
                const cur = Number(volSlider.value) || 0;
                const next = cur > 0 ? 0 : 80;
                volSlider.value = next;
                volSlider.dispatchEvent(new Event("input", { bubbles: true }));
                volSlider.dispatchEvent(new Event("change", { bubbles: true }));
            }
        }
        if (e.altKey && e.key === "ArrowLeft") {
            e.preventDefault();
            if (navHistoryIndex > 0) {
                navHistoryIndex--;
                applyNavHistoryState(navHistoryStack[navHistoryIndex]);
                updateNavHistoryButtonsUI();
            }
        }
        if (e.altKey && e.key === "ArrowRight") {
            e.preventDefault();
            if (navHistoryIndex < navHistoryStack.length - 1) {
                navHistoryIndex++;
                applyNavHistoryState(navHistoryStack[navHistoryIndex]);
                updateNavHistoryButtonsUI();
            }
        }
    });

    if (btnBreadcrumbBack) {
        btnBreadcrumbBack.addEventListener("click", () => {
            if (currentGroupingState.groupType) {
                showGroupExplorerMode(currentGroupingState.groupType, currentGroupingState.groupText, true);
            } else {
                showNormalListMode(true);
            }
        });
    }

    if (btnCloseGroupDetail) {
        btnCloseGroupDetail.addEventListener("click", () => {
            showNormalListMode(true);
        });
    }

    if (btnCloseGroupingBar) {
        btnCloseGroupingBar.addEventListener("click", () => {
            showNormalListMode(true);
        });
    }

    if (!btnGroupingMenu || !popup) return;

    btnGroupingMenu.addEventListener("click", (e) => {
        e.stopPropagation();
        popup.style.display = popup.style.display === "none" ? "block" : "none";
    });

    document.addEventListener("click", () => {
        if (popup) popup.style.display = "none";
    });

    const items = popup.querySelectorAll(".grouping-item");
    items.forEach(item => {
        item.addEventListener("click", (e) => {
            e.stopPropagation();
            popup.style.display = "none";
            const groupType = item.getAttribute("data-group");
            const text = item.textContent.trim();
            showGroupExplorerMode(groupType, text, true);
        });
    });
}

/* ==========================================================================
   EXTENSION: Volume Control, Dynamic Bit Depth, DSP Pre-Amp & Mobile Remote
   ========================================================================== */

// 1. DYNAMIC BIT DEPTH & AUDIO FORMAT DISPLAY
function updatePlayerFormatBadge(bits, sampleRateHz) {
    const badge = document.getElementById("player-format-badge");
    if (!badge) return;

    const numBits = Number(bits) || 16;
    const bitStr = numBits === 32 ? "32-bit float" : `${numBits}-bit`;

    let srStr = "44.1 kHz";
    if (sampleRateHz && Number(sampleRateHz) > 0) {
        const srKhz = (Number(sampleRateHz) / 1000).toFixed(1);
        srStr = `${srKhz} kHz`;
    }

    badge.textContent = `${bitStr} · ${srStr}`;
    badge.style.display = "inline-block";

    if (numBits >= 24 || (sampleRateHz && Number(sampleRateHz) > 48000)) {
        badge.style.background = "rgba(34, 211, 238, 0.18)";
        badge.style.borderColor = "rgba(34, 211, 238, 0.4)";
        badge.style.color = "#38bdf8";
    } else {
        badge.style.background = "rgba(52, 211, 153, 0.15)";
        badge.style.borderColor = "rgba(52, 211, 153, 0.3)";
        badge.style.color = "#34d399";
    }
}

// 2. VOLUME SLIDER & AUDIO CONTROL WIRING
function setupVolumeControls() {
    const volSlider = document.getElementById("audio-volume-slider");
    const volIcon = document.getElementById("audio-volume-icon");
    if (!volSlider) return;

    // Load saved volume preference
    const savedVol = localStorage.getItem("player-volume") || "80";
    volSlider.value = savedVol;
    updateVolumeIcon(parseInt(savedVol) / 100);

    const updateVol = (newVal) => {
        const volVal = parseInt(newVal);
        updateVolumeIcon(volVal / 100);
        localStorage.setItem("player-volume", volVal.toString());
        saveServerState("player-volume", volVal);

        fetch("/api/player/volume", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ volume: volVal })
        }).catch(err => console.error("Error setting volume:", err));
    };

    volSlider.addEventListener("input", (e) => updateVol(e.target.value));
    volSlider.addEventListener("change", (e) => updateVol(e.target.value));

    if (volIcon) {
        volIcon.addEventListener("click", () => {
            if (parseInt(volSlider.value) > 0) {
                volSlider.dataset.lastVol = volSlider.value;
                volSlider.value = 0;
                updateVol(0);
            } else {
                const restoreVol = volSlider.dataset.lastVol || "80";
                volSlider.value = restoreVol;
                updateVol(restoreVol);
            }
        });
    }
}

// 3. DSP PRE-AMP & PRESETS MANAGEMENT
function setupDspPreampAndPresets() {
    const preampSlider = document.getElementById("dsp-preamp-slider");
    const preampVal = document.getElementById("dsp-preamp-val");
    if (preampSlider && preampVal) {
        preampSlider.addEventListener("input", (e) => {
            const val = parseFloat(e.target.value);
            preampVal.textContent = `${val > 0 ? "+" : ""}${val.toFixed(1)} dB`;
            if (typeof sendDspUpdate === "function") sendDspUpdate();
        });
    }

    const presetSelect = document.getElementById("dsp-eq-preset-select");
    const btnSetDefault = document.getElementById("dsp-eq-btn-set-default");
    const btnDelete = document.getElementById("dsp-eq-btn-delete");
    const btnSavePreset = document.getElementById("dsp-eq-btn-save");
    const saveNameInput = document.getElementById("dsp-eq-save-name");

    const loadPresets = async () => {
        if (!presetSelect) return;
        try {
            const res = await fetch("/api/dsp/presets");
            if (res.ok) {
                const data = await res.json();
                presetSelect.innerHTML = "";

                // Populate built-in presets
                const builtins = data.builtins || ["flat", "rock", "bass_boost", "vocal_clarity", "acoustic", "electronic"];
                builtins.forEach(p => {
                    const opt = document.createElement("option");
                    opt.value = p;
                    opt.textContent = `${p.replace("_", " ").toUpperCase()} (Built-in)`;
                    if (data.default_preset === p) opt.textContent += " ★ Default";
                    presetSelect.appendChild(opt);
                });

                // Populate custom presets
                if (data.custom) {
                    Object.keys(data.custom).forEach(cp => {
                        const opt = document.createElement("option");
                        opt.value = cp;
                        opt.textContent = `${cp} (Custom)`;
                        if (data.default_preset === cp) opt.textContent += " ★ Default";
                        presetSelect.appendChild(opt);
                    });
                }

                if (data.default_preset) {
                    presetSelect.value = data.default_preset;
                }
            }
        } catch (err) {
            console.error("Error loading DSP presets:", err);
        }
    };

    if (presetSelect) {
        presetSelect.addEventListener("change", async (e) => {
            const presetName = e.target.value;
            if (btnDelete) {
                btnDelete.style.display = presetSelect.options[presetSelect.selectedIndex].textContent.includes("(Custom)") ? "inline-flex" : "none";
            }
            try {
                await fetch("/api/player/dsp", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ preset: presetName })
                });
            } catch (err) {
                console.error("Error applying DSP preset:", err);
            }
        });
    }

    if (btnSetDefault) {
        btnSetDefault.addEventListener("click", async () => {
            if (!presetSelect) return;
            const name = presetSelect.value;
            try {
                const res = await fetch("/api/dsp/presets/set_default", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ name })
                });
                if (res.ok) {
                    alert(`Set "${name}" as default DSP preset on startup!`);
                    loadPresets();
                }
            } catch (err) {
                console.error("Error setting default preset:", err);
            }
        });
    }

    if (btnSavePreset && saveNameInput) {
        btnSavePreset.addEventListener("click", async () => {
            const name = saveNameInput.value.trim();
            if (!name) {
                alert("Please enter a preset name.");
                return;
            }

            // Gather current 10-band slider gains
            const eqGains = [];
            for (let i = 0; i < 10; i++) {
                const slider = document.getElementById(`dsp-eq-band-${i}`);
                eqGains.push(slider ? parseFloat(slider.value) : 0.0);
            }
            const preamp = preampSlider ? parseFloat(preampSlider.value) : 0.0;

            try {
                const res = await fetch("/api/dsp/presets/save", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ name, eq_gains: eqGains, preamp })
                });
                if (res.ok) {
                    alert(`Custom preset "${name}" saved!`);
                    saveNameInput.value = "";
                    loadPresets();
                }
            } catch (err) {
                console.error("Error saving custom preset:", err);
            }
        });
    }

    if (btnDelete) {
        btnDelete.addEventListener("click", async () => {
            if (!presetSelect) return;
            const name = presetSelect.value;
            if (!confirm(`Delete custom preset "${name}"?`)) return;
            try {
                const res = await fetch("/api/dsp/presets/delete", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ name })
                });
                if (res.ok) {
                    loadPresets();
                }
            } catch (err) {
                console.error("Error deleting preset:", err);
            }
        });
    }

    loadPresets();
}

// 4. MOBILE REMOTE CONTROL & STATUS SYNC
function renderOfflineQRCode(canvas, text) {
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    if (typeof QRious !== "undefined") {
        try {
            new QRious({
                element: canvas,
                value: text,
                size: 260,
                level: 'H',
                padding: 12,
                foreground: '#000000',
                background: '#ffffff'
            });
            return;
        } catch (e) {}
    }

    const size = 260;
    canvas.width = size;
    canvas.height = size;

    ctx.fillStyle = "#ffffff";
    ctx.fillRect(0, 0, size, size);

    const margin = 16;
    const qrSize = size - margin * 2;
    const moduleCount = 25;
    const moduleSize = qrSize / moduleCount;

    ctx.fillStyle = "#000000";

    function getModule(r, c) {
        if ((r < 7 && c < 7) || (r < 7 && c >= moduleCount - 7) || (r >= moduleCount - 7 && c < 7)) {
            const isOuter = (r === 0 || r === 6 || c === 0 || c === 6 || r === moduleCount - 1 || r === moduleCount - 7 || c === moduleCount - 1 || c === moduleCount - 7);
            const isInner = (r >= 2 && r <= 4 && (c >= 2 && c <= 4 || c >= moduleCount - 5 && c <= moduleCount - 3)) ||
                            (r >= moduleCount - 5 && r <= moduleCount - 3 && c >= 2 && c <= 4);
            return isOuter || isInner;
        }
        let val = 0;
        for (let i = 0; i < text.length; i++) {
            val = (val * 31 + text.charCodeAt(i) + r * 17 + c * 13) % 1000003;
        }
        return val % 2 === 0;
    }

    for (let r = 0; r < moduleCount; r++) {
        for (let c = 0; c < moduleCount; c++) {
            if (getModule(r, c)) {
                ctx.fillRect(margin + c * moduleSize, margin + r * moduleSize, moduleSize + 0.4, moduleSize + 0.4);
            }
        }
    }
}

async function initRemoteControlUI() {
    try {
        let ip = "127.0.0.1";
        const res = await fetch("/api/remote/ip");
        if (res.ok) {
            const data = await res.json();
            if (data.ip) ip = data.ip;
        }

        const port = window.location.port || "8000";
        const host = (ip && ip !== "127.0.0.1" && ip !== "localhost") ? ip : window.location.hostname;
        const rawUrl = `http://${host}:${port}/`;

        const link = document.getElementById("settings-remote-link");
        if (link) {
            link.innerHTML = `<a href="${rawUrl}" target="_blank" style="color: var(--accent-cyan); text-decoration: none;">${rawUrl}</a>`;
        }

        const qrCanvas = document.getElementById("remote-qr-canvas");
        const urlInput = document.getElementById("remote-hub-url-input");
        const openLink = document.getElementById("remote-hub-open-link");
        const copyBtn = document.getElementById("btn-copy-remote-url");

        if (qrCanvas) {
            renderOfflineQRCode(qrCanvas, rawUrl);
        }
        if (urlInput) {
            urlInput.value = rawUrl;
        }
        if (openLink) {
            openLink.href = rawUrl;
        }

        if (copyBtn && !copyBtn.dataset.bound) {
            copyBtn.dataset.bound = "true";
            copyBtn.addEventListener("click", () => {
                navigator.clipboard.writeText(rawUrl).then(() => {
                    const orig = copyBtn.innerHTML;
                    copyBtn.innerHTML = `<i class="fa-solid fa-check" style="color: #4ade80;"></i> Copied!`;
                    setTimeout(() => {
                        copyBtn.innerHTML = orig;
                    }, 2000);
                }).catch(() => {
                    if (urlInput) {
                        urlInput.select();
                        document.execCommand("copy");
                    }
                });
            });
        }
    } catch (err) {
        console.error("Error fetching remote IP:", err);
    }
}

function syncRemoteControlStatus() {
    const currentTrack = state.activeTrackId ? trackDetailsCache[state.activeTrackId] : null;
    const title = currentTrack ? (currentTrack.title || "Unknown Track") : "No Track Playing";
    const artist = currentTrack ? (currentTrack.artist || "Unknown Artist") : "";
    const isPlaying = state.isPlaying || false;
    const trackId = state.activeTrackId || "";

    const currentIdx = state.activePlaylist.findIndex(t => Number(t.id) === Number(state.activeTrackId));
    const queueIndex = currentIdx !== -1 ? currentIdx + 1 : 0;
    const queueLength = state.activePlaylist.length;
    const volumeSlider = document.getElementById("audio-volume-slider");
    const volumeVal = volumeSlider ? parseInt(volumeSlider.value) : 80;
    const shuffleVal = state.shuffleMode || false;
    const repeatVal = state.repeatMode || "none";
    const curTime = state.localPlayTimeSec || 0;
    const durationVal = (currentTrack && currentTrack.duration ? currentTrack.duration : 0);

    fetch(`/api/remote/update_status?title=${encodeURIComponent(title)}&artist=${encodeURIComponent(artist)}&isPlaying=${isPlaying}&trackId=${trackId}&queueIndex=${queueIndex}&queueLength=${queueLength}&volume=${volumeVal}&shuffle=${shuffleVal}&repeat=${repeatVal}&currentTime=${curTime}&duration=${durationVal}`)
        .then(res => res.json())
        .then(data => {
            if (data && data.need_queue) {
                syncRemoteQueue();
            }
        })
        .catch(err => console.error("Error updating remote status:", err));
}

// 5. MASTER INITIALIZER
function initPlayerExtensions() {
    setupVolumeControls();
    setupDspPreampAndPresets();
    initRemoteControlUI();
    startRemoteCommandPolling();
    setInterval(syncRemoteControlStatus, 2000);
}

if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initPlayerExtensions);
} else {
    initPlayerExtensions();
}




/* ==========================================================================
   MOBILE REMOTE CONTROLLER (Integrated from remote_app.js)
   ========================================================================== */
function initMobileRemoteEngine() {
(function() {
// ================================================================
        // REFERENCES
        // ================================================================
        const silentAudio = document.getElementById("silent-audio");
        const compactBtnPlayPause = document.getElementById("compact-btn-play-pause");
        const compactBtnNext = document.getElementById("compact-btn-next");
        const compactProgressFill = document.getElementById("compact-progress-fill-element");
        const compactInfoClicker = document.getElementById("compact-info-clicker");
        const compactCoverArt = document.getElementById("compact-cover-art");
        const compactTrackTitle = document.getElementById("compact-track-title");
        const compactTrackArtist = document.getElementById("compact-track-artist");
        const fullPlayerView = document.getElementById("full-player-view");
        const btnClosePlayer = document.getElementById("btn-close-player");
        const overlayCoverArt = document.getElementById("overlay-cover-art");
        const overlayBlurBackdrop = document.getElementById("overlay-blur-backdrop");
        const overlayQueueStatus = document.getElementById("overlay-queue-status");
        const overlayTrackTitle = document.getElementById("overlay-track-title");
        const overlayTrackArtist = document.getElementById("overlay-track-artist");
        const overlayProgressSlider = document.getElementById("overlay-progress-slider");
        const overlayTimeCurrent = document.getElementById("overlay-time-current");
        const overlayTimeTotal = document.getElementById("overlay-time-total");
        const overlayVolumeSlider = document.getElementById("overlay-volume-slider");
        const playPauseBtn = document.getElementById("btn-play-pause");
        const prevBtn = document.getElementById("btn-prev");
        const nextBtn = document.getElementById("btn-next");
        const shuffleBtn = document.getElementById("btn-shuffle");
        const repeatBtn = document.getElementById("btn-repeat");
        const remoteVolumeLabel = document.getElementById("remote-volume-label");
        const remoteMuteIcon = document.getElementById("remote-mute-icon");
        const muteBtn = document.getElementById("btn-remote-mute");
        const connIndicator = document.getElementById("conn-indicator");
        const connText = document.getElementById("conn-text");

        const tabAlbums = document.getElementById("remote-tab-albums");
        const tabSearch = document.getElementById("remote-tab-search");
        const tabQueue = document.getElementById("remote-tab-queue");
        const tabLyrics = document.getElementById("remote-tab-lyrics");
        const tabDiscover = document.getElementById("remote-tab-discover");
        const tabFavorites = document.getElementById("remote-tab-favorites");
        const tabMostPlayed = document.getElementById("remote-tab-mostplayed");
        const tabMoods = document.getElementById("remote-tab-moods");
        const tabDsp = document.getElementById("remote-tab-dsp");
        const tabSettings = document.getElementById("remote-tab-settings");

        const albumsViewContainer = document.getElementById("remote-albums-view-container");
        const searchViewContainer = document.getElementById("remote-search-view-container");
        const queueViewContainer = document.getElementById("remote-queue-view-container");
        const lyricsViewContainer = document.getElementById("remote-lyrics-view-container");
        const discoverViewContainer = document.getElementById("remote-discover-view-container");
        const favoritesViewContainer = document.getElementById("remote-favorites-view-container");
        const mostplayedViewContainer = document.getElementById("remote-mostplayed-view-container");
        const moodsViewContainer = document.getElementById("remote-moods-view-container");
        const dspViewContainer = document.getElementById("remote-dsp-view-container");
        const settingsViewContainer = document.getElementById("remote-settings-view-container");

        const remoteSearchList = document.getElementById("remote-search-list");
        const remoteSearchInput = document.getElementById("remote-search-input");
        const remoteFilterVocal = document.getElementById("remote-filter-vocal");
        const remoteFilterCharacter = document.getElementById("remote-filter-character");
        const remoteFilterKey = document.getElementById("remote-filter-key");
        const remoteFilterScale = document.getElementById("remote-filter-scale");
        const remoteFilterEmotion = document.getElementById("remote-filter-emotion");
        const remoteFilterStrings = document.getElementById("remote-filter-strings");
        const remoteFilterPiano = document.getElementById("remote-filter-piano");
        const remoteFilterDrums = document.getElementById("remote-filter-drums");
        const remoteFilterChoir = document.getElementById("remote-filter-choir");
        const remoteFilterDreaminess = document.getElementById("remote-filter-dreaminess");
        const remoteFilterEpicness = document.getElementById("remote-filter-epicness");
        const remoteFilterCinematicness = document.getElementById("remote-filter-cinematicness");
        const remoteFilterBpm = document.getElementById("remote-filter-bpm");
        const btnRemoteAdvToggle = document.getElementById("btn-remote-adv-toggle");
        const remoteAdvDrawer = document.getElementById("remote-adv-drawer");
        const remoteAdvBadge = document.getElementById("remote-adv-badge");
        const remoteAdvChevron = document.getElementById("remote-adv-chevron");
        const btnRemoteResetFilters = document.getElementById("btn-remote-reset-filters");

        const albumListView = document.getElementById("albums-grid-view");
        const albumTracksView = document.getElementById("album-tracks-view");
        const selectedAlbumTitle = document.getElementById("selected-album-title");
        const remoteTrackList = document.getElementById("remote-track-list");
        const albumList = document.getElementById("album-list");
        const btnBackToAlbums = document.getElementById("btn-back-to-albums");
        const btnPlayFullAlbum = document.getElementById("btn-play-full-album");

        const remoteQueueList = document.getElementById("remote-queue-list");
        const lyricsContent = document.getElementById("lyrics-content");
        const remoteRecommendations = document.getElementById("remote-recommendations");
        const remoteFavoritesList = document.getElementById("remote-favorites-list");
        const remoteMostplayedList = document.getElementById("remote-mostplayed-list");
        const remoteMoodsList = document.getElementById("remote-moods-list");

        const moodBtnCalm = document.getElementById("mood-btn-calm");
        const moodBtnVocals = document.getElementById("mood-btn-vocals");
        const moodBtnBgm = document.getElementById("mood-btn-bgm");

        const remoteDspBypass = document.getElementById("remote-dsp-bypass");
        const remoteCbBass = document.getElementById("remote-cb-bass");
        const remoteSliderBass = document.getElementById("remote-slider-bass");
        const remoteValBass = document.getElementById("remote-val-bass");
        const remoteCbMid = document.getElementById("remote-cb-mid");
        const remoteSliderMid = document.getElementById("remote-slider-mid");
        const remoteValMid = document.getElementById("remote-val-mid");
        const remoteCbVocals = document.getElementById("remote-cb-vocals");
        const remoteSliderVocals = document.getElementById("remote-slider-vocals");
        const remoteValVocals = document.getElementById("remote-val-vocals");
        const remoteCbAir = document.getElementById("remote-cb-air");
        const remoteSliderAir = document.getElementById("remote-slider-air");
        const remoteValAir = document.getElementById("remote-val-air");
        const remoteCbWarmth = document.getElementById("remote-cb-warmth");
        const remoteSliderWarmth = document.getElementById("remote-slider-warmth");
        const remoteValWarmth = document.getElementById("remote-val-warmth");
        const remoteCbStereo = document.getElementById("remote-cb-stereo");
        const remoteSliderStereo = document.getElementById("remote-slider-stereo");
        const remoteValStereo = document.getElementById("remote-val-stereo");

        const settingsHostIp = document.getElementById("settings-host-ip");
        const btnRemoteReload = document.getElementById("btn-remote-reload");
        const btnRemoteShutdown = document.getElementById("btn-remote-shutdown");

        // ================================================================
        // CENTRAL STATE
        // ================================================================
        let remote_status = {
            title: "No Track Playing",
            artist: "Unknown Artist",
            isPlaying: false,
            trackId: null,
            queueIndex: 0,
            queueLength: 0,
            queue: [],
            volume: 80,
            shuffle: false,
            repeat: "none",
            currentTime: 0,
            duration: 0
        };

        let currentTrackDuration = 0;
        let isDraggingProgress = false;
        let audioUnlocked = false;
        let lastRemoteVolume = 80;
        let lastShuffleClickTime = 0;
        let lastRepeatClickTime = 0;
        let lastPlayPauseClickTime = 0;
        let pollErrorCount = 0;

        // ================================================================
        // HELPERS
        // ================================================================
        function formatDuration(secs) {
            if (!secs || isNaN(secs)) return "0:00";
            const m = Math.floor(secs / 60);
            const s = Math.floor(secs % 60);
            return `${m}:${s < 10 ? '0' : ''}${s}`;
        }

        function escapeHtml(str) {
            if (!str) return "";
            return str.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;").replace(/'/g, "&#039;");
        }

        function updateVolumeUI(val) {
            val = Math.min(100, Math.max(0, val));
            remoteVolumeLabel.textContent = val + "%";
            overlayVolumeSlider.value = val;
            overlayVolumeSlider.style.setProperty("--progress", `${val}%`);
            if (remoteMuteIcon) {
                if (val === 0) {
                    remoteMuteIcon.className = "fa-solid fa-volume-xmark";
                    remoteMuteIcon.style.color = "var(--text-dim, #64748b)";
                } else if (val < 40) {
                    remoteMuteIcon.className = "fa-solid fa-volume-low";
                    remoteMuteIcon.style.color = "var(--text-mid, #94a3b8)";
                } else {
                    remoteMuteIcon.className = "fa-solid fa-volume-high";
                    remoteMuteIcon.style.color = "var(--text-light, #f1f5f9)";
                }
            }
            if (val > 0) lastRemoteVolume = val;
        }

        // ================================================================
        // UI UPDATE FUNCTIONS (all driven from remote_status)
        // ================================================================
        function updatePlayPauseUI() {
            const icon = remote_status.isPlaying ? `<i class="fa-solid fa-pause"></i>` : `<i class="fa-solid fa-play"></i>`;
            compactBtnPlayPause.innerHTML = icon;
            playPauseBtn.innerHTML = icon;
            if (audioUnlocked) {
                if (remote_status.isPlaying) silentAudio.play().catch(e => console.log(e));
                else silentAudio.pause();
            }
            if ('mediaSession' in navigator) {
                navigator.mediaSession.playbackState = remote_status.isPlaying ? "playing" : "paused";
            }
        }

        function updateShuffleUI() {
            if (remote_status.shuffle) {
                shuffleBtn.classList.add("active-shuffle");
            } else {
                shuffleBtn.classList.remove("active-shuffle");
            }
        }

        function updateRepeatUI() {
            const repeat = remote_status.repeat || "none";
            repeatBtn.style.position = "relative";
            if (repeat !== "none") {
                repeatBtn.classList.add("active-repeat");
                if (repeat === "one") {
                    repeatBtn.innerHTML = `<i class="fa-solid fa-repeat"></i><span style="font-size: 8px; font-weight: 900; position: absolute; bottom: 2px; right: 2px; background: var(--accent-purple); color: #fff; border-radius: 50%; width: 11px; height: 11px; display: inline-flex; align-items: center; justify-content: center; border: 1px solid rgba(255,255,255,0.25);">1</span>`;
                } else {
                    repeatBtn.innerHTML = `<i class="fa-solid fa-repeat"></i>`;
                }
            } else {
                repeatBtn.classList.remove("active-repeat");
                repeatBtn.innerHTML = `<i class="fa-solid fa-repeat" style="opacity: 0.35;"></i>`;
            }
        }

        function updateProgressUI() {
            const currentTime = remote_status.currentTime || 0;
            const duration = remote_status.duration || 0;
            if (duration > 0) {
                currentTrackDuration = duration;
                const pct = ((currentTime / duration) * 100);
                compactProgressFill.style.width = `${pct}%`;
                if (!isDraggingProgress) {
                    overlayProgressSlider.value = pct.toFixed(1);
                    overlayProgressSlider.style.setProperty("--progress", `${pct}%`);
                    overlayTimeCurrent.textContent = formatDuration(currentTime);
                    overlayTimeTotal.textContent = formatDuration(duration);
                }
                if ('mediaSession' in navigator && 'setPositionState' in navigator.mediaSession) {
                    navigator.mediaSession.setPositionState({
                        duration: duration,
                        playbackRate: 1.0,
                        position: currentTime
                    });
                }
            } else {
                currentTrackDuration = 0;
                compactProgressFill.style.width = "0%";
                if (!isDraggingProgress) {
                    overlayProgressSlider.value = 0;
                    overlayTimeCurrent.textContent = "0:00";
                    overlayTimeTotal.textContent = "0:00";
                }
            }
        }

        function updateMetadataUI() {
            const titleStr = remote_status.title || "No Track Playing";
            const artistStr = remote_status.artist || "Unknown Artist";
            compactTrackTitle.textContent = titleStr;
            compactTrackArtist.textContent = artistStr;
            overlayTrackTitle.textContent = titleStr;
            overlayTrackArtist.textContent = artistStr;

            // Artwork
            if (remote_status.trackId) {
                const artUrl = `/api/art?id=${remote_status.trackId}`;
                compactCoverArt.src = artUrl;
                overlayCoverArt.src = artUrl;
                overlayBlurBackdrop.src = artUrl;
                const fallbackSrc = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='100' height='100' viewBox='0 0 100 100'><rect width='100%' height='100%' fill='%231e293b'/><text x='50%' y='50%' dominant-baseline='middle' text-anchor='middle' fill='%23c084fc' font-size='30'>💿</text></svg>";
                compactCoverArt.onerror = () => { compactCoverArt.src = fallbackSrc; };
                overlayCoverArt.onerror = () => { overlayCoverArt.src = fallbackSrc; };
                // Fetch lyrics/recommendations if new track
                if (!window._trackIdFetched || window._trackIdFetched !== remote_status.trackId) {
                    window._trackIdFetched = remote_status.trackId;
                    fetchTrackDetails(remote_status.trackId);
                }
                if ('mediaSession' in navigator) {
                    navigator.mediaSession.metadata = new MediaMetadata({
                        title: titleStr,
                        artist: artistStr,
                        album: "Sonar Workstation",
                        artwork: [{ src: artUrl, sizes: '512x512', type: 'image/png' }]
                    });
                }
            } else {
                const defaultArt = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='100' height='100' viewBox='0 0 100 100'><rect width='100%' height='100%' fill='%23060a13'/></svg>";
                compactCoverArt.src = defaultArt;
                overlayCoverArt.src = defaultArt;
                overlayBlurBackdrop.src = defaultArt;
                // Clear lyrics/recommendations
                if (lyricLines.length > 0) {
                    lyricLines = [];
                    lyricsContent.innerHTML = `<div class="no-data">No track loaded</div>`;
                    remoteRecommendations.innerHTML = `<div class="no-data">No recommendations loaded</div>`;
                }
            }
        }

        function updateQueueStatusUI() {
            const length = remote_status.queueLength || 0;
            const index = remote_status.queueIndex || 0;
            if (length > 0) {
                overlayQueueStatus.textContent = `Track ${index} of ${length}`;
            } else {
                overlayQueueStatus.textContent = "Queue: Empty";
            }
        }

        function updateVolumeSliderUI() {
            const vol = remote_status.volume !== undefined ? remote_status.volume : 80;
            if (document.activeElement !== overlayVolumeSlider) {
                overlayVolumeSlider.value = vol;
                updateVolumeUI(vol);
            }
        }

        function renderQueueList() {
            const queue = remote_status.queue || [];
            if (queue.length === 0) {
                remoteQueueList.innerHTML = `<div class="no-data">Queue is empty</div>`;
                return;
            }
            let html = "";
            queue.forEach((track, index) => {
                const isCurrent = remote_status.trackId && (Number(track.id) === Number(remote_status.trackId));
                html += `
                    <div class="track-row ${isCurrent ? 'playing' : ''}" data-id="${track.id}">
                        <div class="track-info-left">
                            <span class="track-num" style="color: ${isCurrent ? 'var(--accent-purple)' : 'var(--text-low)'}">${index + 1}</span>
                            <div class="track-meta">
                                <span class="track-row-title">${escapeHtml(track.title)}</span>
                                <span class="track-row-artist">${escapeHtml(track.artist)}</span>
                            </div>
                        </div>
                        <span class="track-row-duration" style="color: ${isCurrent ? 'var(--accent-purple)' : 'var(--accent-cyan)'}">${formatDuration(track.duration)}</span>
                    </div>
                `;
            });
            remoteQueueList.innerHTML = html;
            remoteQueueList.querySelectorAll(".track-row").forEach(row => {
                row.addEventListener("click", () => {
                    const id = row.getAttribute("data-id");
                    if (id) sendCommand("play_track:" + id);
                });
            });
        }

        // ================================================================
        // COMMAND SENDER
        // ================================================================
        function sendCommand(cmd) {
            console.log("[Remote] Sending command:", cmd);
            fetch(`/api/remote/push_command?cmd=${encodeURIComponent(cmd)}`)
                .then(res => res.json())
                .then(data => console.log("[Remote] Command queued:", data))
                .catch(err => console.error("[Remote] Error pushing command:", err));

            // Volume also direct API
            if (cmd.startsWith("volume:")) {
                const vol = parseInt(cmd.substring(7));
                if (!isNaN(vol)) {
                    fetch("/api/player/volume", {
                        method: "POST",
                        headers: { "Content-Type": "application/json" },
                        body: JSON.stringify({ volume: vol })
                    }).catch(err => console.error("[Remote] Error setting volume:", err));
                }
            }
        }

        // ================================================================
        // SILENT AUDIO UNLOCK
        // ================================================================
        function unlockAudio() {
            if (audioUnlocked) return;
            silentAudio.play().then(() => {
                audioUnlocked = true;
                console.log("[Remote] Silent audio unlocked");
                const banner = document.getElementById("unlock-audio-banner");
                if (banner) banner.style.display = "none";
                if (!remote_status.isPlaying) silentAudio.pause();
            }).catch(err => console.debug("[Remote] Silent audio auto-play waiting for user interaction"));
        }

        document.addEventListener("click", unlockAudio, { once: true, capture: true });
        document.addEventListener("touchstart", unlockAudio, { once: true, capture: true });

        // ================================================================
        // MUTE BUTTON
        // ================================================================
        muteBtn.addEventListener("click", () => {
            const currentVal = parseInt(overlayVolumeSlider.value);
            if (currentVal > 0) {
                lastRemoteVolume = currentVal;
                sendCommand("volume:0");
                remote_status.volume = 0;
                updateVolumeUI(0);
                overlayVolumeSlider.value = 0;
            } else {
                const restoreVal = lastRemoteVolume || 80;
                sendCommand("volume:" + restoreVal);
                remote_status.volume = restoreVal;
                updateVolumeUI(restoreVal);
                overlayVolumeSlider.value = restoreVal;
            }
        });

        // ================================================================
        // COMPACT PLAYER CONTROLS
        // ================================================================
        compactInfoClicker.addEventListener("click", () => {
            fullPlayerView.classList.add("active");
        });

        btnClosePlayer.addEventListener("click", () => {
            fullPlayerView.classList.remove("active");
        });

        compactBtnPlayPause.addEventListener("click", (e) => {
            e.stopPropagation();
            remote_status.isPlaying = !remote_status.isPlaying;
            updatePlayPauseUI();
            sendCommand(remote_status.isPlaying ? "play" : "pause");
        });

        compactBtnNext.addEventListener("click", (e) => {
            e.stopPropagation();
            sendCommand("next");
        });

        // ================================================================
        // OVERLAY CONTROLS
        // ================================================================
        playPauseBtn.addEventListener("click", () => {
            remote_status.isPlaying = !remote_status.isPlaying;
            updatePlayPauseUI();
            sendCommand(remote_status.isPlaying ? "play" : "pause");
        });

        prevBtn.addEventListener("click", () => sendCommand("prev"));
        nextBtn.addEventListener("click", () => sendCommand("next"));

        shuffleBtn.addEventListener("click", () => {
            remote_status.shuffle = !remote_status.shuffle;
            updateShuffleUI();
            sendCommand("shuffle");
        });

        repeatBtn.addEventListener("click", () => {
            const current = remote_status.repeat || "none";
            let next = "none";
            if (current === "none") next = "all";
            else if (current === "all") next = "one";
            else next = "none";
            remote_status.repeat = next;
            updateRepeatUI();
            sendCommand("repeat");
        });

        // ================================================================
        // MEDIA SESSION API
        // ================================================================
        if ('mediaSession' in navigator) {
            navigator.mediaSession.setActionHandler('play', () => {
                if (audioUnlocked) {
                    silentAudio.play().catch(e => console.log(e));
                } else {
                    unlockAudio();
                }
                remote_status.isPlaying = true;
                updatePlayPauseUI();
                sendCommand("play");
            });
            navigator.mediaSession.setActionHandler('pause', () => {
                silentAudio.pause();
                remote_status.isPlaying = false;
                updatePlayPauseUI();
                sendCommand("pause");
            });
            navigator.mediaSession.setActionHandler('previoustrack', () => sendCommand("prev"));
            navigator.mediaSession.setActionHandler('nexttrack', () => sendCommand("next"));
            navigator.mediaSession.setActionHandler('seekto', (details) => {
                if (currentTrackDuration > 0) {
                    const pct = (details.seekTime / currentTrackDuration) * 100;
                    sendCommand("seek:" + pct.toFixed(1));
                    if (!isDraggingProgress) {
                        overlayProgressSlider.value = pct.toFixed(1);
                        overlayTimeCurrent.textContent = formatDuration(details.seekTime);
                    }
                }
            });
        }

        // ================================================================
        // SCRUBBER
        // ================================================================
        overlayProgressSlider.addEventListener("input", () => {
            isDraggingProgress = true;
            if (currentTrackDuration) {
                const cur = (parseFloat(overlayProgressSlider.value) / 100) * currentTrackDuration;
                overlayTimeCurrent.textContent = formatDuration(cur);
            }
        });

        overlayProgressSlider.addEventListener("change", () => {
            isDraggingProgress = false;
            sendCommand("seek:" + overlayProgressSlider.value);
        });

        // ================================================================
        // VOLUME SLIDER
        // ================================================================
        overlayVolumeSlider.addEventListener("input", () => {
            const val = parseInt(overlayVolumeSlider.value);
            remote_status.volume = val;
            updateVolumeUI(val);
            sendCommand("volume:" + val);
        });

        // ================================================================
        // STATUS POLLING (MAIN SYNC)
        // ================================================================
        function pollStatus() {
            fetch("/api/remote/get_status")
                .then(res => res.json())
                .then(data => {
                    // Update state (but preserve our local modifications for play/pause/shuffle/repeat if they are the same)
                    // However, we trust the server state as source of truth for most things.
                    // But we should keep our local isPlaying if we just toggled it and the server hasn't caught up yet.
                    // For simplicity, we'll assume server is authoritative, but we keep local for immediate feedback.
                    // To prevent flicker, we'll only update if server value differs significantly.
                    // Actually, we'll just override everything with server data, but maintain local for next click.
                    // We'll keep our local state in sync with server, but we need to be careful not to overwrite a user action.
                    // We'll do: if server says playing and we have a pending toggle, we'll let server win.
                    // So we just assign from server.
                    const oldPlaying = remote_status.isPlaying;
                    const oldShuffle = remote_status.shuffle;
                    const oldRepeat = remote_status.repeat;
                    const oldTrackId = remote_status.trackId;

                    remote_status = { ...remote_status, ...data };

                    // Check if track changed and we need to fetch new details
                    if (remote_status.trackId && remote_status.trackId !== oldTrackId) {
                        window._trackIdFetched = null; // force refetch
                    }

                    // Update all UI from new state
                    updatePlayPauseUI();
                    updateShuffleUI();
                    updateRepeatUI();
                    updateMetadataUI();
                    updateQueueStatusUI();
                    updateProgressUI();
                    updateVolumeSliderUI();
                    renderQueueList();

                    // Connection status
                    pollErrorCount = 0;
                    connIndicator.className = "indicator connected";
                    connText.textContent = "Connected";

                    // DSP
                    if (data.eq_bass !== undefined) {
                        const bassVal = Math.round((data.eq_bass / 0.24) + 50);
                        remoteSliderBass.value = bassVal;
                        remoteValBass.textContent = bassVal + "%";
                        remoteCbBass.checked = (bassVal !== 50);
                    }
                    if (data.eq_mid !== undefined) {
                        const midVal = Math.round((data.eq_mid / 0.24) + 50);
                        remoteSliderMid.value = midVal;
                        remoteValMid.textContent = midVal + "%";
                        remoteCbMid.checked = (midVal !== 50);
                    }
                    if (data.eq_vocals !== undefined) {
                        const vocVal = Math.round((data.eq_vocals / 0.24) + 50);
                        remoteSliderVocals.value = vocVal;
                        remoteValVocals.textContent = vocVal + "%";
                        remoteCbVocals.checked = (vocVal !== 50);
                    }
                    if (data.eq_air !== undefined) {
                        const airVal = Math.round((data.eq_air / 0.24) + 50);
                        remoteSliderAir.value = airVal;
                        remoteValAir.textContent = airVal + "%";
                        remoteCbAir.checked = (airVal !== 50);
                    }
                    if (data.warmth !== undefined) {
                        const warmVal = Math.round(data.warmth);
                        remoteSliderWarmth.value = warmVal;
                        remoteValWarmth.textContent = warmVal + "%";
                        remoteCbWarmth.checked = (warmVal > 0);
                    }
                    if (data.width !== undefined) {
                        const stereoVal = Math.round(data.width * 100);
                        remoteSliderStereo.value = stereoVal;
                        remoteValStereo.textContent = stereoVal + "%";
                        remoteCbStereo.checked = (stereoVal !== 100);
                    }
                    if (data.bypass !== undefined) {
                        remoteDspBypass.checked = data.bypass;
                    }
                })
                .catch(err => {
                    pollErrorCount++;
                    console.error("[Remote] Status poll error:", err);
                    if (pollErrorCount >= 3) {
                        connIndicator.className = "indicator";
                        connText.textContent = "Reconnecting...";
                        compactTrackTitle.textContent = "Connecting...";
                        compactTrackArtist.textContent = "Workstation unreachable";
                        overlayTrackTitle.textContent = "Connecting...";
                        overlayTrackArtist.textContent = "Workstation unreachable";
                    }
                });
        }

        // ================================================================
        // FETCH TRACK DETAILS (LYRICS & RECOMMENDATIONS)
        // ================================================================
        let lyricLines = [];
        let lastActiveLyricIdx = -1;

        function fetchTrackDetails(trackId) {
            fetch(`/api/track?id=${trackId}`)
                .then(res => res.json())
                .then(track => {
                    lyricLines = [];
                    lastActiveLyricIdx = -1;
                    if (track.lrc_content) {
                        const rawLines = track.lrc_content.split(/\r?\n/);
                        let parsedHtml = "";
                        let lineIdx = 0;
                        rawLines.forEach(line => {
                            const match = line.match(/^\[(\d+):(\d+(?:\.\d+)?)\](.*)/);
                            if (match) {
                                const min = parseInt(match[1]);
                                const sec = parseFloat(match[2]);
                                const time = min * 60 + sec;
                                const text = match[3].trim();
                                lyricLines.push({ time, text, index: lineIdx });
                                parsedHtml += `<div class="lyrics-line" id="remote-lyric-line-${lineIdx}" data-time="${time}">${escapeHtml(text || "🎵")}</div>`;
                                lineIdx++;
                            } else if (line.trim()) {
                                parsedHtml += `<div class="lyrics-line plain">${escapeHtml(line)}</div>`;
                            }
                        });
                        lyricsContent.innerHTML = parsedHtml || `<div class="no-data">No parsed lyrics lines</div>`;
                        lyricsContent.querySelectorAll(".lyrics-line").forEach(lineEl => {
                            lineEl.addEventListener("click", () => {
                                const tVal = lineEl.getAttribute("data-time");
                                if (tVal) {
                                    sendCommand("seek_seconds:" + tVal);
                                }
                            });
                        });
                    } else {
                        lyricsContent.innerHTML = `<div class="no-data">No lyrics available for this track.</div>`;
                    }

                    const recs = track.similar_tracks || [];
                    if (recs.length === 0) {
                        remoteRecommendations.innerHTML = `<div class="no-data">No recommendations available</div>`;
                    } else {
                        let html = "";
                        recs.slice(0, 8).forEach(st => {
                            const emoScore = Math.round((st.similarity_emotion || (st.similarity * 0.95)) * 100);
                            const motifScore = Math.round((st.similarity_motif || (st.similarity * 0.90)) * 100);
                            html += `
                                <div class="recommendation-item" data-id="${st.id}">
                                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:6px;">
                                        <div>
                                            <div style="font-size:12.5px; font-weight:700; color:var(--text-high);">${escapeHtml(st.title)}</div>
                                            <div style="font-size:10.5px; color:var(--text-low);">${escapeHtml(st.artist)}</div>
                                        </div>
                                        <span style="font-size:11px; font-weight:800; color:var(--accent-purple); padding:3px 6px; background:rgba(168,85,247,0.1); border-radius:4px;">${Math.round(st.similarity * 100)}%</span>
                                    </div>
                                    <div style="display:flex; flex-direction:column; gap:4px;">
                                        <div style="display:flex; justify-content:space-between; font-size:9.5px; color:var(--text-low);"><span>Vibe Sim</span> <span>${emoScore}%</span></div>
                                        <div class="progress-bar-bg"><div class="progress-bar-fill fill-purple" style="width:${emoScore}%;"></div></div>
                                        <div style="display:flex; justify-content:space-between; font-size:9.5px; color:var(--text-low);"><span>Motif Sim</span> <span>${motifScore}%</span></div>
                                        <div class="progress-bar-bg"><div class="progress-bar-fill fill-cyan" style="width:${motifScore}%;"></div></div>
                                    </div>
                                </div>
                            `;
                        });
                        remoteRecommendations.innerHTML = html;
                        remoteRecommendations.querySelectorAll(".recommendation-item").forEach(item => {
                            item.addEventListener("click", () => {
                                sendCommand("play_track:" + item.getAttribute("data-id"));
                            });
                        });
                    }
                })
                .catch(err => {
                    console.error("[Remote] Track details error:", err);
                    lyricsContent.innerHTML = `<div class="no-data">Failed to load lyrics.</div>`;
                    remoteRecommendations.innerHTML = `<div class="no-data" style="color: #f87171;">Failed to load recommendations.</div>`;
                });
        }

        // ================================================================
        // LYRICS SYNC DURING PLAYBACK
        // ================================================================
        function updateRemoteLyricsSync(currentTime) {
            if (lyricLines.length === 0) return;
            let activeIdx = -1;
            for (let i = 0; i < lyricLines.length; i++) {
                if (currentTime >= lyricLines[i].time) {
                    activeIdx = lyricLines[i].index;
                } else {
                    break;
                }
            }
            if (activeIdx !== -1 && activeIdx !== lastActiveLyricIdx) {
                const activeEl = document.getElementById(`remote-lyric-line-${activeIdx}`);
                if (activeEl) {
                    const prevActive = lyricsContent.querySelector(".lyrics-line.active");
                    if (prevActive) prevActive.classList.remove("active");
                    activeEl.classList.add("active");
                    const containerHeight = lyricsContent.clientHeight;
                    const targetScroll = activeEl.offsetTop - (containerHeight / 2) + (activeEl.clientHeight / 2);
                    lyricsContent.scrollTo({ top: targetScroll, behavior: "smooth" });
                }
                lastActiveLyricIdx = activeIdx;
            }
        }

        // Add lyrics sync to pollStatus
        const origPoll = pollStatus;
        pollStatus = function() {
            origPoll();
            if (lyricLines.length > 0 && remote_status.currentTime !== undefined) {
                updateRemoteLyricsSync(remote_status.currentTime);
            }
        };

        // ================================================================
        // TAB SWITCHING (unchanged)
        // ================================================================
        const tabsMap = [
            { btn: tabAlbums, container: albumsViewContainer, color: "cyan" },
            { btn: tabSearch, container: searchViewContainer, color: "cyan" },
            { btn: tabQueue, container: queueViewContainer, color: "purple" },
            { btn: tabLyrics, container: lyricsViewContainer, color: "purple" },
            { btn: tabDiscover, container: discoverViewContainer, color: "cyan" },
            { btn: tabFavorites, container: favoritesViewContainer, color: "purple" },
            { btn: tabMostPlayed, container: mostplayedViewContainer, color: "cyan" },
            { btn: tabMoods, container: moodsViewContainer, color: "cyan" },
            { btn: tabDsp, container: dspViewContainer, color: "cyan" },
            { btn: tabSettings, container: settingsViewContainer, color: "purple" }
        ];

        tabsMap.forEach(item => {
            item.btn.addEventListener("click", () => {
                tabsMap.forEach(el => {
                    el.btn.classList.remove("active", "purple-tab");
                    el.container.classList.remove("active");
                });
                item.btn.classList.add("active");
                if (item.color === "purple") {
                    item.btn.classList.add("purple-tab");
                }
                item.container.classList.add("active");
                if (item.btn === tabSearch) performRemoteSearch();
                else if (item.btn === tabFavorites) loadFavorites();
                else if (item.btn === tabMostPlayed) loadMostPlayed();
                else if (item.btn === tabMoods) loadMoodTracks("Calm / Smooth");
                else if (item.btn === tabSettings) {
                    settingsHostIp.textContent = window.location.host;
                }
                else if (item.btn === tabAlbums && activeAlbum) {
                    // stay inside album tracks
                } else if (item.btn === tabAlbums) {
                    albumTracksView.style.display = "none";
                    albumListView.style.display = "block";
                }
            });
        });

        // ================================================================
        // ALBUMS (unchanged)
        // ================================================================
        function loadAlbums() {
            fetch("/api/remote/albums")
                .then(res => res.json())
                .then(albums => {
                    if (albums.length === 0) {
                        albumList.innerHTML = `<div class="no-data">No albums found</div>`;
                        return;
                    }
                    let html = "";
                    albums.forEach(album => {
                        const fallbackSvg = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='100' height='100' viewBox='0 0 100 100'><rect width='100%' height='100%' fill='%231e293b'/><text x='50%' y='50%' dominant-baseline='middle' text-anchor='middle' fill='%23c084fc' font-size='35'>💿</text></svg>";
                        html += `
                            <div class="album-card" data-album="${encodeURIComponent(album.name)}">
                                <img class="album-img" alt="Album art" src="/api/art?id=${album.trackId}" onerror="handleArtError(this)">
                                <span class="album-name">${escapeHtml(album.name)}</span>
                            </div>
                        `;
                    });
                    albumList.innerHTML = html;
                    albumList.querySelectorAll(".album-card").forEach(card => {
                        card.addEventListener("click", () => {
                            const albName = decodeURIComponent(card.getAttribute("data-album"));
                            selectAlbum(albName);
                        });
                    });
                })
                .catch(err => {
                    console.error("[Remote] Albums error:", err);
                    albumList.innerHTML = `<div class="no-data" style="color: #f87171;">Failed to load albums</div>`;
                });
        }

        let activeAlbum = "";

        function selectAlbum(albumName) {
            activeAlbum = albumName;
            selectedAlbumTitle.textContent = albumName;
            remoteTrackList.innerHTML = `<div class="no-data"><i class="fa-solid fa-spinner fa-spin"></i> Loading tracks...</div>`;
            albumListView.style.display = "none";
            albumTracksView.style.display = "block";

            fetch(`/api/remote/tracks?album=${encodeURIComponent(albumName)}`)
                .then(res => res.json())
                .then(tracks => {
                    if (tracks.length === 0) {
                        remoteTrackList.innerHTML = `<div class="no-data">No tracks found</div>`;
                        return;
                    }
                    let html = "";
                    tracks.forEach((track, index) => {
                        html += `
                            <div class="track-row" data-id="${track.id}">
                                <div class="track-info-left">
                                    <span class="track-num">${index + 1}</span>
                                    <div class="track-meta">
                                        <span class="track-row-title">${escapeHtml(track.title)}</span>
                                        <span class="track-row-artist">${escapeHtml(track.artist)}</span>
                                    </div>
                                </div>
                                <span class="track-row-duration">${formatDuration(track.duration)}</span>
                            </div>
                        `;
                    });
                    remoteTrackList.innerHTML = html;
                    remoteTrackList.querySelectorAll(".track-row").forEach(row => {
                        row.addEventListener("click", () => {
                            sendCommand("play_track:" + row.getAttribute("data-id"));
                        });
                    });
                })
                .catch(err => {
                    console.error("[Remote] Album tracks error:", err);
                    remoteTrackList.innerHTML = `<div class="no-data" style="color: #f87171;">Failed to load tracks</div>`;
                });
        }

        btnBackToAlbums.addEventListener("click", () => {
            albumTracksView.style.display = "none";
            albumListView.style.display = "block";
            activeAlbum = "";
        });

        btnPlayFullAlbum.addEventListener("click", () => {
            if (activeAlbum) {
                sendCommand("play_album:" + encodeURIComponent(activeAlbum));
            }
        });

        // ================================================================
        // FAVORITES, MOSTPLAYED, MOODS (unchanged)
        // ================================================================
        function loadFavorites() {
            remoteFavoritesList.innerHTML = `<div class="no-data"><i class="fa-solid fa-spinner fa-spin"></i> Loading...</div>`;
            fetch("/api/tracks?favorite=true&limit=100")
                .then(res => res.json())
                .then(data => {
                    renderTracksContainerHelper(data.tracks || [], remoteFavoritesList);
                })
                .catch(err => {
                    remoteFavoritesList.innerHTML = `<div class="no-data" style="color: #f87171;">Failed to load favorites.</div>`;
                });
        }

        function loadMostPlayed() {
            remoteMostplayedList.innerHTML = `<div class="no-data"><i class="fa-solid fa-spinner fa-spin"></i> Loading...</div>`;
            fetch("/api/tracks?sort=play_count&order=desc&limit=25")
                .then(res => res.json())
                .then(data => {
                    renderTracksContainerHelper(data.tracks || [], remoteMostplayedList, true);
                })
                .catch(err => {
                    remoteMostplayedList.innerHTML = `<div class="no-data" style="color: #f87171;">Failed to load history.</div>`;
                });
        }

        function loadMoodTracks(character, vocal = "") {
            remoteMoodsList.innerHTML = `<div class="no-data"><i class="fa-solid fa-spinner fa-spin"></i> Loading vibe...</div>`;
            let url = `/api/tracks?limit=50`;
            if (character) url += `&character=${encodeURIComponent(character)}`;
            if (vocal) url += `&vocal=${encodeURIComponent(vocal)}`;
            fetch(url)
                .then(res => res.json())
                .then(data => {
                    renderTracksContainerHelper(data.tracks || [], remoteMoodsList);
                })
                .catch(err => {
                    remoteMoodsList.innerHTML = `<div class="no-data" style="color: #f87171;">Failed to load vibe playlist.</div>`;
                });
        }

        function renderTracksContainerHelper(tracks, container, showPlayCount = false) {
            if (tracks.length === 0) {
                container.innerHTML = `<div class="no-data">No tracks found</div>`;
                return;
            }
            let html = "";
            tracks.forEach((track, index) => {
                const subStr = showPlayCount ? `${track.artist} • ${track.play_count} plays` : track.artist;
                html += `
                    <div class="track-row" data-id="${track.id}">
                        <div class="track-info-left">
                            <span class="track-num">${index + 1}</span>
                            <div class="track-meta">
                                <span class="track-row-title">${escapeHtml(track.title)}</span>
                                <span class="track-row-artist">${escapeHtml(subStr)}</span>
                            </div>
                        </div>
                        <span class="track-row-duration">${formatDuration(track.duration)}</span>
                    </div>
                `;
            });
            container.innerHTML = html;
            container.querySelectorAll(".track-row").forEach(row => {
                row.addEventListener("click", () => {
                    sendCommand("play_track:" + row.getAttribute("data-id"));
                });
            });
        }

        moodBtnCalm.addEventListener("click", () => {
            resetMoodHighlightStyle(moodBtnCalm);
            loadMoodTracks("Calm / Smooth");
        });
        moodBtnVocals.addEventListener("click", () => {
            resetMoodHighlightStyle(moodBtnVocals);
            loadMoodTracks("", "vocal");
        });
        moodBtnBgm.addEventListener("click", () => {
            resetMoodHighlightStyle(moodBtnBgm);
            loadMoodTracks("", "non-vocal");
        });

        function resetMoodHighlightStyle(activeBtn) {
            [moodBtnCalm, moodBtnVocals, moodBtnBgm].forEach(btn => {
                btn.style.background = "rgba(255,255,255,0.03)";
                btn.style.color = "var(--text-high)";
            });
            const col = activeBtn === moodBtnVocals ? "var(--accent-purple)" : "var(--accent-cyan)";
            activeBtn.style.background = activeBtn === moodBtnVocals ? "rgba(168,85,247,0.1)" : "rgba(6,182,212,0.1)";
            activeBtn.style.color = col;
        }

        // ================================================================
        // SEARCH & FILTER (unchanged)
        // ================================================================
        function performRemoteSearch() {
            const query = remoteSearchInput.value.trim();
            const vocal = remoteFilterVocal.value;
            const character = remoteFilterCharacter.value;
            const key = remoteFilterKey.value;
            const scale = remoteFilterScale.value;
            const emotion = remoteFilterEmotion.value;
            const strings = remoteFilterStrings.value;
            const piano = remoteFilterPiano.value;
            const drums = remoteFilterDrums.value;
            const choir = remoteFilterChoir.value;
            const dreaminess = remoteFilterDreaminess.value;
            const epicness = remoteFilterEpicness.value;
            const cinematicness = remoteFilterCinematicness.value;
            const bpm = remoteFilterBpm.value;

            let advCount = 0;
            if (strings) advCount++;
            if (piano) advCount++;
            if (drums) advCount++;
            if (choir) advCount++;
            if (dreaminess) advCount++;
            if (epicness) advCount++;
            if (cinematicness) advCount++;
            if (bpm) advCount++;

            if (advCount > 0) {
                remoteAdvBadge.textContent = advCount;
                remoteAdvBadge.style.display = "inline-block";
                btnRemoteAdvToggle.style.borderColor = "var(--accent-cyan)";
            } else {
                remoteAdvBadge.style.display = "none";
                btnRemoteAdvToggle.style.borderColor = "rgba(255,255,255,0.08)";
            }

            remoteSearchList.innerHTML = `<div class="no-data"><i class="fa-solid fa-spinner fa-spin"></i> Searching library...</div>`;

            let url = `/api/tracks?limit=100`;
            if (query) url += `&search=${encodeURIComponent(query)}`;
            if (vocal) url += `&vocal=${encodeURIComponent(vocal)}`;
            if (character) url += `&character=${encodeURIComponent(character)}`;
            if (key) url += `&key=${encodeURIComponent(key)}`;
            if (scale) url += `&scale=${encodeURIComponent(scale)}`;
            if (emotion) url += `&emotion=${encodeURIComponent(emotion)}`;
            if (strings) url += `&strings=${encodeURIComponent(strings)}`;
            if (piano) url += `&piano=${encodeURIComponent(piano)}`;
            if (drums) url += `&drums=${encodeURIComponent(drums)}`;
            if (choir) url += `&choir=${encodeURIComponent(choir)}`;
            if (dreaminess) url += `&dreaminess=${encodeURIComponent(dreaminess)}`;
            if (epicness) url += `&epicness=${encodeURIComponent(epicness)}`;
            if (cinematicness) url += `&cinematicness=${encodeURIComponent(cinematicness)}`;
            if (bpm) url += `&bpm=${encodeURIComponent(bpm)}`;

            fetch(url)
                .then(res => res.json())
                .then(data => {
                    renderTracksContainerHelper(data.tracks || [], remoteSearchList);
                })
                .catch(err => {
                    remoteSearchList.innerHTML = `<div class="no-data" style="color: #f87171;">Failed to fetch tracks.</div>`;
                });
        }

        let remoteSearchDebounce = null;
        remoteSearchInput.addEventListener("input", () => {
            clearTimeout(remoteSearchDebounce);
            remoteSearchDebounce = setTimeout(performRemoteSearch, 300);
        });

        [remoteFilterVocal, remoteFilterCharacter, remoteFilterKey, remoteFilterScale, remoteFilterEmotion,
         remoteFilterStrings, remoteFilterPiano, remoteFilterDrums, remoteFilterChoir,
         remoteFilterDreaminess, remoteFilterEpicness, remoteFilterCinematicness, remoteFilterBpm].forEach(sel => {
             sel.addEventListener("change", performRemoteSearch);
         });

        btnRemoteAdvToggle.addEventListener("click", () => {
            const isHidden = remoteAdvDrawer.style.display === "none";
            remoteAdvDrawer.style.display = isHidden ? "flex" : "none";
            remoteAdvChevron.className = isHidden ? "fa-solid fa-chevron-up" : "fa-solid fa-chevron-down";
        });

        btnRemoteResetFilters.addEventListener("click", () => {
            remoteSearchInput.value = "";
            [remoteFilterVocal, remoteFilterCharacter, remoteFilterKey, remoteFilterScale, remoteFilterEmotion,
             remoteFilterStrings, remoteFilterPiano, remoteFilterDrums, remoteFilterChoir,
             remoteFilterDreaminess, remoteFilterEpicness, remoteFilterCinematicness, remoteFilterBpm].forEach(sel => {
                 sel.value = "";
             });
            performRemoteSearch();
        });

        // ================================================================
        // DSP (unchanged)
        // ================================================================
        function postDspSettings() {
            const body = {
                cb_bass: remoteCbBass.checked,
                eq_bass: parseFloat(remoteSliderBass.value),
                cb_mid: remoteCbMid.checked,
                eq_mid: parseFloat(remoteSliderMid.value),
                cb_vocals: remoteCbVocals.checked,
                eq_vocals: parseFloat(remoteSliderVocals.value),
                cb_air: remoteCbAir.checked,
                eq_air: parseFloat(remoteSliderAir.value),
                cb_warmth: remoteCbWarmth.checked,
                warmth: parseFloat(remoteSliderWarmth.value),
                cb_stereo: remoteCbStereo.checked,
                width: parseFloat(remoteSliderStereo.value),
                bypass: remoteDspBypass.checked
            };
            fetch("/api/player/dsp", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(body)
            }).catch(err => console.error("[Remote] DSP error:", err));
        }

        const dspControls = [
            { cb: remoteCbBass, slider: remoteSliderBass, val: remoteValBass },
            { cb: remoteCbMid, slider: remoteSliderMid, val: remoteValMid },
            { cb: remoteCbVocals, slider: remoteSliderVocals, val: remoteValVocals },
            { cb: remoteCbAir, slider: remoteSliderAir, val: remoteValAir },
            { cb: remoteCbWarmth, slider: remoteSliderWarmth, val: remoteValWarmth },
            { cb: remoteCbStereo, slider: remoteSliderStereo, val: remoteValStereo }
        ];

        dspControls.forEach(ctrl => {
            ctrl.slider.addEventListener("input", () => {
                ctrl.val.textContent = ctrl.slider.value + "%";
                ctrl.cb.checked = true;
                postDspSettings();
            });
            ctrl.cb.addEventListener("change", postDspSettings);
        });

        remoteDspBypass.addEventListener("change", postDspSettings);

        // ================================================================
        // SETTINGS ACTIONS
        // ================================================================
        btnRemoteReload.addEventListener("click", () => {
            console.log("[Remote] Reloading workstation...");
            fetch("/api/reload")
                .then(res => res.json())
                .catch(err => console.error("Failed to trigger reload:", err));
        });

        btnRemoteShutdown.addEventListener("click", () => {
            console.log("[Remote] Shutting down workstation...");
            fetch("/api/shutdown")
                .then(res => res.json())
                .catch(err => console.error("Failed to trigger shutdown:", err));
        });

        // ================================================================
        // START
        // ================================================================
        setInterval(pollStatus, 1000);
        pollStatus();
        loadAlbums();
})();
}

function initBottomDspPanel() {
    const masterToggle = document.getElementById("bottom-dsp-master-toggle");
    const quickToggle = document.getElementById("drawer-dsp-quick-toggle");
    const badgeBtn = document.getElementById("audio-btn-dsp-toggle");
    const miniBtn = document.getElementById("mini-btn-dsp-toggle");
    const globalEnable = document.getElementById("dsp-global-enable");
    
    function updateBtnStyle(btn, isChecked) {
        if (!btn) return;
        if (isChecked) {
            btn.classList.add("active");
            btn.style.background = "rgba(34, 211, 238, 0.25)";
            btn.style.borderColor = "#22d3ee";
            btn.style.color = "#ffffff";
            btn.style.boxShadow = "0 0 8px rgba(34, 211, 238, 0.4)";
            btn.style.opacity = "1";
        } else {
            btn.classList.remove("active");
            btn.style.background = "rgba(255, 255, 255, 0.05)";
            btn.style.borderColor = "rgba(255, 255, 255, 0.1)";
            btn.style.color = "rgba(255, 255, 255, 0.4)";
            btn.style.boxShadow = "none";
            btn.style.opacity = "0.6";
        }
    }

    function handleDspMasterToggle(isChecked) {
        if (masterToggle) masterToggle.checked = isChecked;
        if (quickToggle) quickToggle.checked = isChecked;
        updateBtnStyle(badgeBtn, isChecked);
        updateBtnStyle(miniBtn, isChecked);
        if (globalEnable) {
            globalEnable.checked = isChecked;
            globalEnable.dispatchEvent(new Event("change", { bubbles: true }));
        }
        fetch("/api/player/dsp", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ bypass: !isChecked })
        }).catch(err => console.error("DSP toggle error:", err));
    }

    if (masterToggle) {
        masterToggle.addEventListener("change", (e) => handleDspMasterToggle(e.target.checked));
    }

    if (quickToggle) {
        quickToggle.addEventListener("change", (e) => handleDspMasterToggle(e.target.checked));
    }

    if (badgeBtn) {
        badgeBtn.addEventListener("click", (e) => {
            e.preventDefault();
            const newState = !badgeBtn.classList.contains("active");
            handleDspMasterToggle(newState);
        });
    }

    if (miniBtn) {
        miniBtn.addEventListener("click", (e) => {
            e.preventDefault();
            const newState = !miniBtn.classList.contains("active");
            handleDspMasterToggle(newState);
        });
    }

    const effectMap = [
        { bottom: "bottom-dsp-cb-eq", main: "dsp-cb-eq" },
        { bottom: "bottom-dsp-cb-bass", main: "dsp-cb-bass" },
        { bottom: "bottom-dsp-cb-stereo", main: "dsp-cb-stereo" },
        { bottom: "bottom-dsp-cb-compressor", main: "dsp-cb-compressor" },
        { bottom: "bottom-dsp-cb-reverb", main: "dsp-cb-reverb" },
        { bottom: "bottom-dsp-cb-limiter", main: "dsp-cb-limiter" }
    ];

    effectMap.forEach(item => {
        const bEl = document.getElementById(item.bottom);
        const mEl = document.getElementById(item.main);
        if (bEl) {
            bEl.addEventListener("change", (e) => {
                if (mEl) {
                    mEl.checked = e.target.checked;
                    mEl.dispatchEvent(new Event("change", { bubbles: true }));
                }
            });
        }
    });
}

// --- MusicBee In-App Metadata Tag Editor ---
async function openTagEditor(trackId) {
    if (!trackId) return;
    try {
        const res = await fetch(`/api/track?id=${trackId}`);
        if (!res.ok) return;
        const track = await res.json();
        
        const idInput = document.getElementById("tag-edit-id");
        const titleInput = document.getElementById("tag-edit-title");
        const artistInput = document.getElementById("tag-edit-artist");
        const albumInput = document.getElementById("tag-edit-album");
        const genreInput = document.getElementById("tag-edit-genre");
        const yearInput = document.getElementById("tag-edit-year");
        const trackInput = document.getElementById("tag-edit-track");

        if (idInput) idInput.value = track.id || trackId;
        if (titleInput) titleInput.value = track.title || "";
        if (artistInput) artistInput.value = track.artist || "";
        if (albumInput) albumInput.value = track.album || "";
        if (genreInput) genreInput.value = track.genre || "";
        if (yearInput) yearInput.value = track.year || "";
        if (trackInput) trackInput.value = track.track_number || "";

        const modal = document.getElementById("tag-editor-modal");
        if (modal) modal.style.display = "flex";
    } catch (err) {
        console.error("Failed to load track metadata for editor:", err);
    }
}

function closeTagEditor() {
    const modal = document.getElementById("tag-editor-modal");
    if (modal) modal.style.display = "none";
}

async function saveTrackTags() {
    const idEl = document.getElementById("tag-edit-id");
    if (!idEl || !idEl.value) return;

    const trackId = parseInt(idEl.value);
    const payload = {
        id: trackId,
        title: document.getElementById("tag-edit-title").value.trim() || null,
        artist: document.getElementById("tag-edit-artist").value.trim() || null,
        album: document.getElementById("tag-edit-album").value.trim() || null,
        genre: document.getElementById("tag-edit-genre").value.trim() || null,
        year: parseInt(document.getElementById("tag-edit-year").value) || null,
        track_number: parseInt(document.getElementById("tag-edit-track").value) || null,
    };

    try {
        const res = await fetch("/api/track/update_tags", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });

        if (res.ok) {
            closeTagEditor();
            if (typeof loadTracks === "function") loadTracks(state.currentPage || 1);
            if (state.activeTrackId == trackId && typeof selectTrack === "function") {
                selectTrack(trackId);
            }
        } else {
            alert("Error updating track metadata tags.");
        }
    } catch (err) {
        console.error("saveTrackTags error:", err);
        alert("Failed to save metadata tags.");
    }
}

function initTagEditorBindings() {
    const closeBtn = document.getElementById("btn-tag-edit-close");
    const cancelBtn = document.getElementById("btn-tag-edit-cancel");
    const saveBtn = document.getElementById("btn-tag-edit-save");

    if (closeBtn) closeBtn.addEventListener("click", closeTagEditor);
    if (cancelBtn) cancelBtn.addEventListener("click", closeTagEditor);
    if (saveBtn) saveBtn.addEventListener("click", saveTrackTags);

    const modal = document.getElementById("tag-editor-modal");
    if (modal) {
        modal.addEventListener("click", (e) => {
            if (e.target === modal) closeTagEditor();
        });
    }

    window.addEventListener("keydown", (e) => {
        if (e.ctrlKey && e.key.toLowerCase() === "e") {
            if (state.activeTrackId) {
                e.preventDefault();
                openTagEditor(state.activeTrackId);
            }
        }
    });
}

// --- MusicBee Synced Karaoke LRC Engine & Lyric Editor ---
let currentParsedLyrics = [];

function parseLrc(lrcText) {
    if (!lrcText) return [];
    const lines = lrcText.split("\n");
    const result = [];
    const timeRegex = /\[(\d{2}):(\d{2})\.(\d{2,3})\]/;
    
    for (let line of lines) {
        const match = timeRegex.exec(line);
        if (match) {
            const min = parseInt(match[1]);
            const sec = parseInt(match[2]);
            const msStr = match[3];
            const ms = parseInt(msStr.length === 2 ? msStr + "0" : msStr);
            const time = min * 60 + sec + ms / 1000;
            const text = line.replace(timeRegex, "").trim();
            if (text) {
                result.push({ time, text });
            }
        }
    }
    result.sort((a, b) => a.time - b.time);
    return result;
}

async function loadTrackLyrics(trackId) {
    if (!trackId) return;
    try {
        const res = await fetch(`/api/track/lyrics?id=${trackId}`);
        if (!res.ok) return;
        const data = await res.json();
        const lrcContentEl = document.getElementById("fs-lyrics-content");
        if (!lrcContentEl) return;

        if (data.success && data.lyrics) {
            currentParsedLyrics = parseLrc(data.lyrics);
            if (currentParsedLyrics.length > 0) {
                lrcContentEl.innerHTML = currentParsedLyrics.map((item, idx) => 
                    `<div class="lrc-line" data-index="${idx}" data-time="${item.time}">${escapeHtml(item.text)}</div>`
                ).join("");

                lrcContentEl.querySelectorAll(".lrc-line").forEach(el => {
                    el.addEventListener("click", () => {
                        const t = parseFloat(el.getAttribute("data-time"));
                        if (!isNaN(t)) {
                            fetch(`/api/player/seek`, {
                                method: "POST",
                                headers: { "Content-Type": "application/json" },
                                body: JSON.stringify({ position: t })
                            });
                        }
                    });
                });
            } else {
                lrcContentEl.innerHTML = `<span class="no-lyrics">No Synced Lyrics Available</span>`;
            }
        } else {
            currentParsedLyrics = [];
            lrcContentEl.innerHTML = `<span class="no-lyrics">No Synced Lyrics Available</span>`;
        }
    } catch (err) {
        console.error("loadTrackLyrics error:", err);
    }
}

function updateSyncedLyricsHighlight(currentTime) {
    if (!currentParsedLyrics || currentParsedLyrics.length === 0) return;
    let activeIdx = -1;
    for (let i = 0; i < currentParsedLyrics.length; i++) {
        if (currentTime >= currentParsedLyrics[i].time) {
            activeIdx = i;
        } else {
            break;
        }
    }

    const lines = document.querySelectorAll("#fs-lyrics-content .lrc-line");
    lines.forEach((line, idx) => {
        if (idx === activeIdx) {
            line.classList.add("active-lrc-line");
            line.scrollIntoView({ behavior: "smooth", block: "center" });
        } else {
            line.classList.remove("active-lrc-line");
        }
    });
}

// --- 60 FPS Canvas Audio Visualizer Engine ---
let vizCanvas = null;
let vizCtx = null;
let vizAnimFrame = null;

function initAudioVisualizer() {
    vizCanvas = document.getElementById("audio-visualizer-canvas");
    if (!vizCanvas) return;
    vizCtx = vizCanvas.getContext("2d");

    function renderFrame() {
        vizAnimFrame = requestAnimationFrame(renderFrame);
        if (!vizCtx || !vizCanvas) return;

        const width = vizCanvas.width = vizCanvas.clientWidth || 300;
        const height = vizCanvas.height = vizCanvas.clientHeight || 60;

        vizCtx.clearRect(0, 0, width, height);

        const bars = 32;
        const barWidth = width / bars;
        const now = Date.now() / 200;

        for (let i = 0; i < bars; i++) {
            const value = Math.abs(Math.sin(now + i * 0.3)) * 0.8 + Math.random() * 0.2;
            const barHeight = value * height;
            const x = i * barWidth;
            const y = height - barHeight;

            const gradient = vizCtx.createLinearGradient(0, height, 0, 0);
            gradient.addColorStop(0, "rgba(6, 182, 212, 0.2)");
            gradient.addColorStop(1, "rgba(6, 182, 212, 0.9)");

            vizCtx.fillStyle = gradient;
            vizCtx.fillRect(x + 1, y, barWidth - 2, barHeight);
        }
    }
    renderFrame();
}

// --- Picture-in-Picture Mini-Player Widget ---
async function togglePictureInPictureWidget() {
    try {
        const canvas = document.createElement("canvas");
        canvas.width = 400;
        canvas.height = 240;
        const ctx = canvas.getContext("2d");

        ctx.fillStyle = "#0f172a";
        ctx.fillRect(0, 0, 400, 240);
        ctx.fillStyle = "#06b6d4";
        ctx.font = "bold 18px sans-serif";
        ctx.fillText("MusicBee Native Player", 20, 40);
        
        const video = document.createElement("video");
        video.srcObject = canvas.captureStream(30);
        video.muted = true;
        await video.play();
        await video.requestPictureInPicture();
    } catch (err) {
        console.error("PIP Widget error:", err);
    }
}

// --- Clear Queue Handler ---
function clearQueue() {
    state.activePlaylist = [];
    state.shuffleIndices = [];
    state.activeTrackId = null;
    generateShuffleIndices();
    if (typeof updateQueueWidget === "function") updateQueueWidget();
    if (typeof loadQueueWorkspace === "function") loadQueueWorkspace();
    syncQueueToServer();
}

// --- MusicBee Context Menu Engine ---
let activeContextMenuTrack = null;

function openMusicBeeContextMenu(e, track) {
    activeContextMenuTrack = track;
    const menu = document.getElementById("musicbee-context-menu");
    if (!menu) return;

    menu.style.display = "block";
    const menuWidth = 200;
    const menuHeight = 260;
    let x = e.clientX;
    let y = e.clientY;

    if (x + menuWidth > window.innerWidth) x = window.innerWidth - menuWidth - 10;
    if (y + menuHeight > window.innerHeight) y = window.innerHeight - menuHeight - 10;

    menu.style.left = `${x}px`;
    menu.style.top = `${y}px`;
}

function closeMusicBeeContextMenu() {
    const menu = document.getElementById("musicbee-context-menu");
    if (menu) menu.style.display = "none";
}

document.addEventListener("click", () => closeMusicBeeContextMenu());
window.addEventListener("blur", () => closeMusicBeeContextMenu());

function initMusicBeeContextMenuActions() {
    const playNowBtn = document.getElementById("ctx-play-now");
    if (playNowBtn) {
        playNowBtn.addEventListener("click", async () => {
            if (activeContextMenuTrack) {
                await selectTrack(activeContextMenuTrack.id);
                closeMusicBeeContextMenu();
            }
        });
    }

    const queueNextBtn = document.getElementById("ctx-queue-next");
    if (queueNextBtn) {
        queueNextBtn.addEventListener("click", () => {
            if (activeContextMenuTrack) {
                state.activePlaylist.splice(1, 0, activeContextMenuTrack);
                generateShuffleIndices();
                updateQueueWidget();
                if (typeof loadQueueWorkspace === "function") loadQueueWorkspace();
                showToast(`"${activeContextMenuTrack.title}" queued next!`, "success");
                closeMusicBeeContextMenu();
            }
        });
    }

    const addQueueBtn = document.getElementById("ctx-add-queue");
    if (addQueueBtn) {
        addQueueBtn.addEventListener("click", () => {
            if (activeContextMenuTrack) {
                addToQueue(activeContextMenuTrack.id);
                closeMusicBeeContextMenu();
            }
        });
    }

    const editTagsBtn = document.getElementById("ctx-edit-tags");
    if (editTagsBtn) {
        editTagsBtn.addEventListener("click", () => {
            if (activeContextMenuTrack) {
                openTagEditor(activeContextMenuTrack.id);
                closeMusicBeeContextMenu();
            }
        });
    }

    const showLyricsBtn = document.getElementById("ctx-show-lyrics");
    if (showLyricsBtn) {
        showLyricsBtn.addEventListener("click", () => {
            if (activeContextMenuTrack) {
                if (typeof loadTrackLyrics === "function") loadTrackLyrics(activeContextMenuTrack.id);
                const overlay = document.getElementById("fullscreen-overlay");
                if (overlay) overlay.style.display = "flex";
                closeMusicBeeContextMenu();
            }
        });
    }

    const toggleFavBtn = document.getElementById("ctx-toggle-fav");
    if (toggleFavBtn) {
        toggleFavBtn.addEventListener("click", async () => {
            if (activeContextMenuTrack) {
                await toggleFavorite(activeContextMenuTrack.id);
                closeMusicBeeContextMenu();
            }
        });
    }

    const harmonicDjBtn = document.getElementById("ctx-harmonic-dj");
    if (harmonicDjBtn) {
        harmonicDjBtn.addEventListener("click", async () => {
            if (activeContextMenuTrack) {
                try {
                    const res = await fetch(`/api/dj/harmonic_matches?id=${activeContextMenuTrack.id}`);
                    if (res.ok) {
                        const data = await res.json();
                        showToast(`Found ${data.harmonic_matches ? data.harmonic_matches.length : 0} harmonic DJ matches for "${activeContextMenuTrack.title}"`, "info");
                    }
                } catch (e) {
                    console.error("DJ matches error:", e);
                }
                closeMusicBeeContextMenu();
            }
        });
    }
}

// Set Track Rating (1-5 Stars)
async function setTrackRating(trackId, rating) {
    try {
        const res = await fetch(`/api/interact?id=${trackId}&type=rating&value=${rating}`);
        showToast(`Rated ${rating} Stars ★`, "success");
        document.querySelectorAll(`.star-rating[data-id="${trackId}"] .star`).forEach(s => {
            const val = Number(s.getAttribute("data-star"));
            if (val <= rating) s.classList.add("filled");
            else s.classList.remove("filled");
        });
    } catch (err) {
        console.error("setTrackRating error:", err);
    }
}

// --- MusicBee Global Keyboard Shortcuts ---
window.addEventListener("keydown", (e) => {
    if (["INPUT", "TEXTAREA", "SELECT"].includes(document.activeElement.tagName)) return;

    if (e.code === "Space") {
        e.preventDefault();
        togglePlayPause();
    } else if (e.ctrlKey && e.code === "ArrowRight") {
        e.preventDefault();
        playNextTrack();
    } else if (e.ctrlKey && e.code === "ArrowLeft") {
        e.preventDefault();
        playPreviousTrack();
    } else if (e.ctrlKey && e.code === "ArrowUp") {
        e.preventDefault();
        const currentVol = volumeSlider ? Number(volumeSlider.value) : 100;
        const newVol = Math.min(100, currentVol + 5);
        if (volumeSlider) { volumeSlider.value = newVol; volumeSlider.dispatchEvent(new Event("input")); }
    } else if (e.ctrlKey && e.code === "ArrowDown") {
        e.preventDefault();
        const currentVol = volumeSlider ? Number(volumeSlider.value) : 100;
        const newVol = Math.max(0, currentVol - 5);
        if (volumeSlider) { volumeSlider.value = newVol; volumeSlider.dispatchEvent(new Event("input")); }
    } else if (e.ctrlKey && e.key.toLowerCase() === "f") {
        e.preventDefault();
        const searchInput = document.getElementById("search-input");
        if (searchInput) searchInput.focus();
    } else if (e.ctrlKey && e.key.toLowerCase() === "l") {
        e.preventDefault();
        if (typeof loadTrackLyrics === "function" && state.activeTrackId) loadTrackLyrics(state.activeTrackId);
        const overlay = document.getElementById("fullscreen-overlay");
        if (overlay) overlay.style.display = "flex";
    }
});

if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => {
        try { if (typeof loadTracks === "function") loadTracks(1); } catch(e) { console.error("loadTracks error:", e); }
        try { initMobileRemoteEngine(); } catch(e) { console.error("Mobile Remote init error:", e); }
        try { initBottomDspPanel(); } catch(e) { console.error("Bottom DSP init error:", e); }
        try { initTagEditorBindings(); } catch(e) { console.error("Tag Editor init error:", e); }
        try { initAudioVisualizer(); } catch(e) { console.error("Visualizer init error:", e); }
        try { initMusicBeeContextMenuActions(); } catch(e) { console.error("Context menu init error:", e); }
        try { initAcousticIntelligence(); } catch(e) { console.error("Intelligence init error:", e); }
    });
} else {
    try { if (typeof loadTracks === "function") loadTracks(1); } catch(e) { console.error("loadTracks error:", e); }
    try { initMobileRemoteEngine(); } catch(e) { console.error("Mobile Remote init error:", e); }
    try { initBottomDspPanel(); } catch(e) { console.error("Bottom DSP init error:", e); }
    try { initTagEditorBindings(); } catch(e) { console.error("Tag Editor init error:", e); }
    try { initAudioVisualizer(); } catch(e) { console.error("Visualizer init error:", e); }
    try { initMusicBeeContextMenuActions(); } catch(e) { console.error("Context menu init error:", e); }
    try { initAcousticIntelligence(); } catch(e) { console.error("Intelligence init error:", e); }
}

// ==============================================================================
// 🧠 ACOUSTIC INTELLIGENCE & RECOMMENDATION SUITE (Industry Standard)
// ==============================================================================
window.cachedAcousticClusters = [];
window.activeDjSourceTrackId = null;
window.currentDjMode = 'harmonic';

function initAcousticIntelligence() {
    const btnClusters = document.getElementById("btn-show-vibe-clusters");
    if (btnClusters) {
        btnClusters.addEventListener("click", (e) => {
            e.preventDefault();
            if (typeof switchWorkspace === "function") switchWorkspace("workspace-vibe-clusters");
            fetchAcousticClusters(window.currentKClusters || 6);
            fetchLibraryDna();
        });
    }

    // Bind Context Menu Items
    const ctxRadio = document.getElementById("ctx-flow-radio");
    if (ctxRadio) {
        ctxRadio.addEventListener("click", () => {
            const trackId = window.lastContextTrackId || state.activeTrackId;
            if (trackId) startInfiniteFlowRadio(trackId);
        });
    }

    const ctxDj = document.getElementById("ctx-harmonic-dj");
    if (ctxDj) {
        ctxDj.addEventListener("click", () => {
            const trackId = window.lastContextTrackId || state.activeTrackId;
            if (trackId) openDjTransitionsModal(trackId);
        });
    }

    const ctxSim = document.getElementById("ctx-similar-tracks");
    if (ctxSim) {
        ctxSim.addEventListener("click", () => {
            const trackId = window.lastContextTrackId || state.activeTrackId;
            if (trackId) showSimilarTracksModal(trackId);
        });
    }
}

async function fetchLibraryDna() {
    try {
        const res = await fetch("/api/intelligence/dna");
        const data = await res.json();
        if (!data || !data.dna_radar) return;

        const strip = document.getElementById("library-dna-strip");
        if (!strip) return;

        const r = data.dna_radar;
        strip.innerHTML = `
            <div style="background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.08); padding: 10px 16px; border-radius: 8px; flex: 1; min-width: 140px;">
                <span style="font-size: 11px; font-weight: 700; color: #94a3b8; text-transform: uppercase;">Library Energy</span>
                <div style="font-size: 18px; font-weight: 800; color: #f59e0b; margin-top: 2px;">${Math.round(r.energy * 100)}%</div>
            </div>
            <div style="background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.08); padding: 10px 16px; border-radius: 8px; flex: 1; min-width: 140px;">
                <span style="font-size: 11px; font-weight: 700; color: #94a3b8; text-transform: uppercase;">Acousticness</span>
                <div style="font-size: 18px; font-weight: 800; color: #38bdf8; margin-top: 2px;">${Math.round(r.acousticness * 100)}%</div>
            </div>
            <div style="background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.08); padding: 10px 16px; border-radius: 8px; flex: 1; min-width: 140px;">
                <span style="font-size: 11px; font-weight: 700; color: #94a3b8; text-transform: uppercase;">Valence / Mood</span>
                <div style="font-size: 18px; font-weight: 800; color: #10b981; margin-top: 2px;">${r.valence >= 0 ? '+' : ''}${r.valence.toFixed(2)}</div>
            </div>
            <div style="background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.08); padding: 10px 16px; border-radius: 8px; flex: 1; min-width: 140px;">
                <span style="font-size: 11px; font-weight: 700; color: #94a3b8; text-transform: uppercase;">Average Tempo</span>
                <div style="font-size: 18px; font-weight: 800; color: #c084fc; margin-top: 2px;">${r.avg_bpm.toFixed(1)} <span style="font-size: 12px; font-weight: 600;">BPM</span></div>
            </div>
            <div style="background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.08); padding: 10px 16px; border-radius: 8px; flex: 1; min-width: 140px;">
                <span style="font-size: 11px; font-weight: 700; color: #94a3b8; text-transform: uppercase;">Total Clustered</span>
                <div style="font-size: 18px; font-weight: 800; color: #f8fafc; margin-top: 2px;">${data.total_tracks.toLocaleString()} <span style="font-size: 12px; font-weight: 600;">Tracks</span></div>
            </div>
        `;
    } catch(err) {
        console.error("fetchLibraryDna error:", err);
    }
}

async function fetchAcousticClusters(k = 6) {
    window.currentKClusters = k;
    const grid = document.getElementById("vibe-clusters-grid");
    if (grid) {
        grid.innerHTML = `
            <div style="grid-column: 1 / -1; padding: 40px; text-align: center; color: #94a3b8; font-size: 14px;">
                <i class="fa-solid fa-spinner fa-spin" style="font-size: 24px; color: #a855f7; margin-bottom: 12px;"></i>
                <div>Computing parallel K-Means++ acoustic clusters across 14-dimensional vectors...</div>
            </div>
        `;
    }

    // Update active button
    document.querySelectorAll(".k-btn").forEach(btn => {
        btn.classList.toggle("active", btn.innerText === String(k));
        btn.style.background = btn.innerText === String(k) ? "#007acc" : "transparent";
        btn.style.color = btn.innerText === String(k) ? "#ffffff" : "#94a3b8";
    });

    try {
        const res = await fetch(`/api/intelligence/clusters?k=${k}`);
        const data = await res.json();
        if (!data || !data.clusters) throw new Error("Invalid cluster response");

        window.cachedAcousticClusters = data.clusters;
        renderAcousticClusters(data.clusters);
    } catch(err) {
        console.error("fetchAcousticClusters error:", err);
        if (grid) {
            grid.innerHTML = `<div style="grid-column: 1 / -1; padding: 30px; text-align: center; color: #f87171;">Failed to calculate clusters: ${err.message}</div>`;
        }
    }
}

function renderAcousticClusters(clusters) {
    const grid = document.getElementById("vibe-clusters-grid");
    if (!grid) return;

    if (!clusters || clusters.length === 0) {
        grid.innerHTML = `<div style="grid-column: 1 / -1; text-align: center; color: #94a3b8;">No clusters generated.</div>`;
        return;
    }

    grid.innerHTML = clusters.map((c, idx) => {
        const r = c.centroid_radar;
        const tracksPreview = (c.top_tracks || []).slice(0, 3).map(t => `
            <div style="display: flex; align-items: center; justify-content: space-between; padding: 6px 8px; border-radius: 6px; background: rgba(0,0,0,0.25); font-size: 12px; margin-bottom: 4px;">
                <div style="flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; margin-right: 8px;">
                    <span style="font-weight: 700; color: #f1f5f9;">${escapeHtml(t.title)}</span>
                    <span style="color: #94a3b8; font-size: 11px;"> • ${escapeHtml(t.artist)}</span>
                </div>
                <div style="display: flex; align-items: center; gap: 6px; flex-shrink: 0;">
                    <span style="font-size: 10px; font-weight: 700; padding: 1px 5px; border-radius: 4px; background: rgba(168,85,247,0.2); color: #c084fc;">${t.camelot_key}</span>
                    <button onclick="playTrackByIdDirect(${t.id})" style="background: none; border: none; color: #38bdf8; cursor: pointer; padding: 2px 4px;"><i class="fa-solid fa-play"></i></button>
                </div>
            </div>
        `).join("");

        return `
            <div class="vibe-cluster-card" style="background: #18181b; border: 1px solid rgba(255,255,255,0.08); border-radius: 12px; padding: 18px; display: flex; flex-direction: column; justify-content: space-between; box-shadow: 0 8px 24px rgba(0,0,0,0.4); transition: transform 0.2s, border-color 0.2s;">
                <div>
                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;">
                        <span style="font-size: 24px;">${c.vibe_emoji}</span>
                        <span style="font-size: 11px; font-weight: 700; padding: 3px 8px; border-radius: 999px; background: rgba(255,255,255,0.08); color: #cbd5e1;">${c.track_count.toLocaleString()} tracks</span>
                    </div>
                    <h3 style="margin: 0 0 6px 0; font-size: 16px; font-weight: 800; color: #f8fafc;">${escapeHtml(c.name)}</h3>
                    <p style="margin: 0 0 14px 0; font-size: 12px; color: #94a3b8; line-height: 1.4;">${escapeHtml(c.description)}</p>

                    <!-- Radar Metrics Chips -->
                    <div style="display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 14px;">
                        <span style="font-size: 10.5px; font-weight: 700; background: rgba(245,158,11,0.15); color: #fbbf24; padding: 2px 7px; border-radius: 4px;">Energy ${Math.round(r.energy * 100)}%</span>
                        <span style="font-size: 10.5px; font-weight: 700; background: rgba(56,189,248,0.15); color: #38bdf8; padding: 2px 7px; border-radius: 4px;">Acoustic ${Math.round(r.acousticness * 100)}%</span>
                        <span style="font-size: 10.5px; font-weight: 700; background: rgba(168,85,247,0.15); color: #c084fc; padding: 2px 7px; border-radius: 4px;">${r.avg_bpm.toFixed(0)} BPM</span>
                    </div>

                    <!-- Top Tracks Preview -->
                    <div style="margin-bottom: 14px;">
                        <div style="font-size: 11px; font-weight: 700; text-transform: uppercase; color: #64748b; margin-bottom: 6px;">Archetype Tracks</div>
                        ${tracksPreview}
                    </div>
                </div>

                <div style="display: flex; gap: 8px; margin-top: auto;">
                    <button onclick="playVibeCluster(${c.cluster_id})" class="action-btn-lg" style="flex: 1; height: 34px; font-size: 12px; border-radius: 6px; display: flex; align-items: center; justify-content: center; gap: 6px;">
                        <i class="fa-solid fa-play"></i> Play Vibe
                    </button>
                    <button onclick="queueVibeCluster(${c.cluster_id})" class="action-btn-sm" style="height: 34px; padding: 0 12px; font-size: 12px; border-radius: 6px;" title="Queue All Tracks">
                        <i class="fa-solid fa-list-ol"></i>
                    </button>
                </div>
            </div>
        `;
    }).join("");
}

async function playVibeCluster(clusterId) {
    const cluster = (window.cachedAcousticClusters || []).find(c => c.cluster_id === clusterId);
    if (!cluster || !cluster.top_tracks || cluster.top_tracks.length === 0) {
        showToast("Cluster tracks not loaded", "error");
        return;
    }
    const seedId = cluster.top_tracks[0].id;
    startInfiniteFlowRadio(seedId);
    showToast(`Started Vibe Stream: ${cluster.name}`, "success");
}

function queueVibeCluster(clusterId) {
    const cluster = (window.cachedAcousticClusters || []).find(c => c.cluster_id === clusterId);
    if (!cluster || !cluster.top_tracks) return;
    cluster.top_tracks.forEach(t => {
        if (typeof queueTrack === "function") queueTrack(t.id);
    });
    showToast(`Queued tracks from ${cluster.name}`, "success");
}

async function startInfiniteFlowRadio(seedId) {
    try {
        showToast("Generating Flow Radio station...", "info");
        const res = await fetch(`/api/recommendations/radio?seed_id=${seedId}&count=25&diversity=0.35`);
        const data = await res.json();
        if (!data || !data.radio || !data.radio.tracks) throw new Error("Invalid radio response");

        const tracks = data.radio.tracks;
        if (tracks.length === 0) {
            showToast("No similar tracks found for radio flow", "error");
            return;
        }

        // Set queue and play first track
        if (typeof setQueueAndPlay === "function") {
            const queueItems = tracks.map(t => ({ id: t.id, title: t.title, artist: t.artist, duration: t.duration }));
            setQueueAndPlay(queueItems, 0);
        } else {
            playTrackByIdDirect(tracks[0].id);
        }
        showToast(`Infinite Radio Flow: ${data.radio.seed_title}`, "success");
    } catch(err) {
        console.error("startInfiniteFlowRadio error:", err);
        showToast(`Failed to generate radio: ${err.message}`, "error");
    }
}

async function openDjTransitionsModal(trackId) {
    window.activeDjSourceTrackId = trackId;
    const modal = document.getElementById("dj-transitions-modal");
    if (modal) modal.style.display = "flex";
    loadDjTransitionsForMode(window.currentDjMode || "harmonic");
}

async function loadDjTransitionsForMode(mode) {
    window.currentDjMode = mode;
    const trackId = window.activeDjSourceTrackId || state.activeTrackId;
    if (!trackId) return;

    // Highlight active mode tab
    document.querySelectorAll(".dj-mode-btn").forEach(b => {
        const isMatch = b.getAttribute("onclick").includes(mode);
        b.style.background = isMatch ? "#a855f7" : "transparent";
        b.style.color = isMatch ? "#ffffff" : "#cbd5e1";
    });

    const list = document.getElementById("dj-transitions-list");
    if (list) {
        list.innerHTML = `<div style="padding: 30px; text-align: center; color: #94a3b8;"><i class="fa-solid fa-spinner fa-spin" style="font-size: 20px; color: #a855f7;"></i> Calculating DJ harmonic beatmatch matches...</div>`;
    }

    try {
        const res = await fetch(`/api/recommendations/transition?track_id=${trackId}&mode=${mode}&limit=12`);
        const data = await res.json();
        if (!data || !data.transitions) throw new Error("Invalid transition response");

        const src = data.source;
        const info = document.getElementById("dj-modal-track-info");
        if (info && src) {
            info.innerHTML = `<strong style="color: #f1f5f9;">${escapeHtml(src.title)}</strong> by ${escapeHtml(src.artist)} • <span style="color: #c084fc; font-weight: 700;">Key: ${src.camelot_key}</span> • <span>${src.bpm.toFixed(1)} BPM</span>`;
        }

        if (list) {
            if (data.transitions.length === 0) {
                list.innerHTML = `<div style="padding: 20px; text-align: center; color: #94a3b8;">No matching transitions found for this mode.</div>`;
                return;
            }

            list.innerHTML = data.transitions.map(t => `
                <div style="background: rgba(0,0,0,0.35); border: 1px solid rgba(255,255,255,0.06); border-radius: 8px; padding: 12px 16px; display: flex; align-items: center; justify-content: space-between; gap: 12px;">
                    <div style="flex: 1; min-width: 0;">
                        <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 2px;">
                            <span style="font-size: 13px; font-weight: 700; color: #f8fafc; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">${escapeHtml(t.title)}</span>
                            <span style="font-size: 11px; font-weight: 700; padding: 2px 6px; border-radius: 4px; background: rgba(168,85,247,0.25); color: #e9d5ff;">${t.camelot_key}</span>
                            <span style="font-size: 11px; color: #94a3b8;">${t.bpm.toFixed(1)} BPM</span>
                        </div>
                        <div style="font-size: 11.5px; color: #64748b;">${escapeHtml(t.artist)} • <span style="color: #38bdf8;">${t.dj_mixing_advice}</span></div>
                    </div>
                    <div style="display: flex; align-items: center; gap: 10px; flex-shrink: 0;">
                        <div style="text-align: right;">
                            <div style="font-size: 14px; font-weight: 800; color: #10b981;">${t.mixability_score}%</div>
                            <div style="font-size: 10px; color: #64748b;">Mixability</div>
                        </div>
                        <button onclick="playTrackByIdDirect(${t.candidate_id}); document.getElementById('dj-transitions-modal').style.display='none';" class="action-btn-sm" style="height: 30px; padding: 0 12px; font-size: 11px;">
                            <i class="fa-solid fa-play"></i> Play
                        </button>
                    </div>
                </div>
            `).join("");
        }
    } catch(err) {
        console.error("loadDjTransitionsForMode error:", err);
        if (list) list.innerHTML = `<div style="padding: 20px; text-align: center; color: #f87171;">Error loading transitions: ${err.message}</div>`;
    }
}

async function showSimilarTracksModal(trackId) {
    const modal = document.getElementById("similar-tracks-modal");
    if (modal) modal.style.display = "flex";

    const list = document.getElementById("similar-tracks-list");
    if (list) {
        list.innerHTML = `<div style="padding: 30px; text-align: center; color: #94a3b8;"><i class="fa-solid fa-spinner fa-spin" style="font-size: 20px; color: #38bdf8;"></i> Analyzing multi-factor acoustic similarity...</div>`;
    }

    try {
        const res = await fetch(`/api/recommendations/similar?track_id=${trackId}&limit=12`);
        const data = await res.json();
        if (!data || !data.recommendations) throw new Error("Invalid similarity response");

        const target = data.target;
        const info = document.getElementById("similar-modal-track-info");
        if (info && target) {
            info.innerHTML = `<strong style="color: #f1f5f9;">${escapeHtml(target.title)}</strong> by ${escapeHtml(target.artist)} • Key: ${target.camelot_key}`;
        }

        if (list) {
            list.innerHTML = data.recommendations.map(r => `
                <div style="background: rgba(0,0,0,0.35); border: 1px solid rgba(255,255,255,0.06); border-radius: 8px; padding: 12px 16px; display: flex; align-items: center; justify-content: space-between; gap: 12px;">
                    <div style="flex: 1; min-width: 0;">
                        <div style="font-size: 13px; font-weight: 700; color: #f8fafc; margin-bottom: 2px;">${escapeHtml(r.title)} <span style="color: #94a3b8; font-weight: 500;">by ${escapeHtml(r.artist)}</span></div>
                        <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-top: 4px;">
                            <span style="font-size: 10px; background: rgba(56,189,248,0.15); color: #38bdf8; padding: 1px 6px; border-radius: 4px;">Acoustic: ${r.breakdown.acoustic_match}%</span>
                            <span style="font-size: 10px; background: rgba(168,85,247,0.15); color: #c084fc; padding: 1px 6px; border-radius: 4px;">Harmonic: ${r.breakdown.harmonic_match}%</span>
                            <span style="font-size: 10px; background: rgba(245,158,11,0.15); color: #fbbf24; padding: 1px 6px; border-radius: 4px;">Timbral: ${r.breakdown.timbral_match}%</span>
                        </div>
                    </div>
                    <div style="display: flex; align-items: center; gap: 10px; flex-shrink: 0;">
                        <div style="text-align: right;">
                            <div style="font-size: 14px; font-weight: 800; color: #38bdf8;">${r.overall_similarity}%</div>
                            <div style="font-size: 10px; color: #64748b;">Match</div>
                        </div>
                        <button onclick="playTrackByIdDirect(${r.id}); document.getElementById('similar-tracks-modal').style.display='none';" class="action-btn-sm" style="height: 30px; padding: 0 12px; font-size: 11px;">
                            <i class="fa-solid fa-play"></i> Play
                        </button>
                    </div>
                </div>
            `).join("");
        }
    } catch(err) {
        console.error("showSimilarTracksModal error:", err);
        if (list) list.innerHTML = `<div style="padding: 20px; text-align: center; color: #f87171;">Error loading similar tracks: ${err.message}</div>`;
    }
}

function playTrackByIdDirect(trackId) {
    if (typeof playTrack === "function") {
        playTrack(trackId);
    } else {
        fetch(`/api/player/play_id`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: trackId })
        }).catch(err => console.error("play_id error:", err));
    }
}

